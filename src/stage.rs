//! Pipeline stages for record processing.
//!
//! Stages are the building blocks of pipelines. Each stage transforms,
//! filters, or otherwise processes records as they flow through.
//!
//! ## Common Mainframe Operations
//!
//! - **Filter**: Select records matching criteria (like DFSORT INCLUDE/OMIT)
//! - **Select**: Extract specific columns from records
//! - **Reformat**: Rearrange fields into a new layout (like DFSORT OUTREC)

use crate::Record;

/// A pipeline stage that processes records.
///
/// Stages can transform, filter, or otherwise modify records. The `process`
/// method returns `Some(record)` to pass a record downstream, or `None` to
/// filter it out.
pub trait Stage {
    /// Process a single record.
    ///
    /// Returns `Some(record)` to pass the record downstream, or `None` to
    /// filter it out.
    fn process(&mut self, record: Record) -> Option<Record>;

    /// Process a batch of records.
    ///
    /// Default implementation processes records one at a time.
    fn process_batch(&mut self, records: Vec<Record>) -> Vec<Record> {
        records
            .into_iter()
            .filter_map(|r| self.process(r))
            .collect()
    }
}

/// Filter stage - selects records matching a predicate.
///
/// This is similar to DFSORT's INCLUDE/OMIT operations.
///
/// # Example
///
/// ```
/// use pipelines_rs::{Record, Stage, Filter};
///
/// let mut filter = Filter::new(|r: &Record| r.field_eq(18, 10, "SALES"));
///
/// let sales = Record::from_str("SMITH   JOHN      SALES     ");
/// let eng = Record::from_str("JONES   MARY      ENGINEERING");
///
/// assert!(filter.process(sales).is_some());
/// assert!(filter.process(eng).is_none());
/// ```
pub struct Filter<F>
where
    F: FnMut(&Record) -> bool,
{
    predicate: F,
}

impl<F> Filter<F>
where
    F: FnMut(&Record) -> bool,
{
    /// Creates a new filter stage with the given predicate.
    pub fn new(predicate: F) -> Self {
        Self { predicate }
    }
}

impl<F> Stage for Filter<F>
where
    F: FnMut(&Record) -> bool,
{
    fn process(&mut self, record: Record) -> Option<Record> {
        if (self.predicate)(&record) {
            Some(record)
        } else {
            None
        }
    }
}

/// Select stage - extracts specific fields from records.
///
/// Creates a new record containing only the selected fields.
/// Fields are specified as (source_start, length, dest_start) tuples.
///
/// # Example
///
/// ```
/// use pipelines_rs::{Record, Stage, Select};
///
/// // Select last name (0-8) and department (18-28) into new positions
/// let mut select = Select::new(vec![
///     (0, 8, 0),    // Last name -> position 0
///     (18, 10, 8),  // Department -> position 8
/// ]);
///
/// let input = Record::from_str("SMITH   JOHN      SALES     00050000");
/// let output = select.process(input).unwrap();
///
/// assert_eq!(output.field(0, 8).trim(), "SMITH");
/// assert_eq!(output.field(8, 10).trim(), "SALES");
/// ```
pub struct Select {
    /// Fields to select: (source_start, length, dest_start)
    fields: Vec<(usize, usize, usize)>,
}

impl Select {
    /// Creates a new select stage.
    ///
    /// # Arguments
    ///
    /// * `fields` - Vector of (source_start, length, dest_start) tuples
    pub fn new(fields: Vec<(usize, usize, usize)>) -> Self {
        Self { fields }
    }
}

impl Stage for Select {
    fn process(&mut self, record: Record) -> Option<Record> {
        let mut output = Record::new();

        for &(src_start, length, dest_start) in &self.fields {
            let value = record.field(src_start, length);
            output.set_field(dest_start, length, value);
        }

        Some(output)
    }
}

/// Reformat stage - transforms records using a custom function.
///
/// This is the most flexible stage, allowing arbitrary record transformation.
/// Similar to DFSORT's OUTREC operation.
///
/// # Example
///
/// ```
/// use pipelines_rs::{Record, Stage, Reformat};
///
/// let mut reformat = Reformat::new(|r| {
///     let mut output = Record::new();
///     // Copy name, convert to uppercase department
///     output.set_field(0, 8, r.field(0, 8));
///     output.set_field(8, 10, &r.field(18, 10).to_uppercase());
///     output
/// });
///
/// let input = Record::from_str("SMITH   JOHN      sales     ");
/// let output = reformat.process(input).unwrap();
///
/// assert_eq!(output.field(8, 10).trim(), "SALES");
/// ```
pub struct Reformat<F>
where
    F: FnMut(&Record) -> Record,
{
    transform: F,
}

impl<F> Reformat<F>
where
    F: FnMut(&Record) -> Record,
{
    /// Creates a new reformat stage with the given transform function.
    pub fn new(transform: F) -> Self {
        Self { transform }
    }
}

impl<F> Stage for Reformat<F>
where
    F: FnMut(&Record) -> Record,
{
    fn process(&mut self, record: Record) -> Option<Record> {
        Some((self.transform)(&record))
    }
}

/// Map stage - transforms each record, potentially filtering.
///
/// Like Reformat but returns Option to allow filtering during transformation.
pub struct Map<F>
where
    F: FnMut(Record) -> Option<Record>,
{
    transform: F,
}

impl<F> Map<F>
where
    F: FnMut(Record) -> Option<Record>,
{
    /// Creates a new map stage.
    pub fn new(transform: F) -> Self {
        Self { transform }
    }
}

impl<F> Stage for Map<F>
where
    F: FnMut(Record) -> Option<Record>,
{
    fn process(&mut self, record: Record) -> Option<Record> {
        (self.transform)(record)
    }
}

/// Inspect stage - observes records without modifying them.
///
/// Useful for debugging or logging.
///
/// # Example
///
/// ```
/// use pipelines_rs::{Record, Stage};
/// use pipelines_rs::stage::Inspect;
///
/// let mut count = 0;
/// let mut inspect = Inspect::new(|r: &Record| {
///     println!("Processing: {}", r.field(0, 8).trim());
/// });
/// ```
pub struct Inspect<F>
where
    F: FnMut(&Record),
{
    callback: F,
}

impl<F> Inspect<F>
where
    F: FnMut(&Record),
{
    /// Creates a new inspect stage.
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

impl<F> Stage for Inspect<F>
where
    F: FnMut(&Record),
{
    fn process(&mut self, record: Record) -> Option<Record> {
        (self.callback)(&record);
        Some(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_records() -> Vec<Record> {
        vec![
            Record::from_str("SMITH   JOHN      SALES     00050000"),
            Record::from_str("JONES   MARY      ENGINEERING00075000"),
            Record::from_str("DOE     JANE      SALES     00060000"),
            Record::from_str("WILSON  BOB       MARKETING 00055000"),
        ]
    }

    #[test]
    fn test_filter_include() {
        let mut filter = Filter::new(|r: &Record| r.field_eq(18, 10, "SALES"));
        let records = sample_records();

        let result: Vec<_> = records
            .into_iter()
            .filter_map(|r| filter.process(r))
            .collect();

        assert_eq!(result.len(), 2);
        assert!(result[0].field_eq(0, 8, "SMITH"));
        assert!(result[1].field_eq(0, 8, "DOE"));
    }

    #[test]
    fn test_filter_omit() {
        // Omit SALES (keep non-SALES)
        let mut filter = Filter::new(|r: &Record| !r.field_eq(18, 10, "SALES"));
        let records = sample_records();

        let result: Vec<_> = records
            .into_iter()
            .filter_map(|r| filter.process(r))
            .collect();

        assert_eq!(result.len(), 2);
        assert!(result[0].field_eq(0, 8, "JONES"));
        assert!(result[1].field_eq(0, 8, "WILSON"));
    }

    #[test]
    fn test_select_fields() {
        let mut select = Select::new(vec![
            (0, 8, 0),   // Last name
            (18, 10, 8), // Department
        ]);

        let input = Record::from_str("SMITH   JOHN      SALES     00050000");
        let output = select.process(input).unwrap();

        assert_eq!(output.field(0, 8).trim(), "SMITH");
        assert_eq!(output.field(8, 10).trim(), "SALES");
        // Rest should be blank
        assert!(output.field(18, 62).trim().is_empty());
    }

    #[test]
    fn test_reformat() {
        let mut reformat = Reformat::new(|r: &Record| {
            let mut output = Record::new();
            // Swap first and last name positions
            output.set_field(0, 10, r.field(8, 10)); // First name first
            output.set_field(10, 8, r.field(0, 8)); // Last name second
            output.set_field(18, 10, r.field(18, 10)); // Keep department
            output
        });

        let input = Record::from_str("SMITH   JOHN      SALES     ");
        let output = reformat.process(input).unwrap();

        assert_eq!(output.field(0, 10).trim(), "JOHN");
        assert_eq!(output.field(10, 8).trim(), "SMITH");
        assert_eq!(output.field(18, 10).trim(), "SALES");
    }

    #[test]
    fn test_inspect() {
        let mut seen = Vec::new();
        {
            let mut inspect = Inspect::new(|r: &Record| {
                seen.push(r.field(0, 8).trim().to_string());
            });

            for record in sample_records() {
                inspect.process(record);
            }
        }

        assert_eq!(seen, vec!["SMITH", "JONES", "DOE", "WILSON"]);
    }

    #[test]
    fn test_process_batch() {
        let mut filter = Filter::new(|r: &Record| r.field_eq(18, 10, "SALES"));
        let result = filter.process_batch(sample_records());

        assert_eq!(result.len(), 2);
    }
}
