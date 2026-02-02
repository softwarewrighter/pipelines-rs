//! Pipeline orchestration for record processing.
//!
//! The `Pipeline` struct provides a fluent API for building record processing
//! workflows, similar to mainframe JCL and DFSORT operations.
//!
//! # Example
//!
//! ```
//! use pipelines_rs::{Pipeline, Record};
//!
//! let records = vec![
//!     Record::from_str("SMITH   JOHN      SALES     00050000"),
//!     Record::from_str("JONES   MARY      ENGINEERING00075000"),
//!     Record::from_str("DOE     JANE      SALES     00060000"),
//! ];
//!
//! let result: Vec<Record> = Pipeline::new(records.into_iter())
//!     .filter(|r| r.field_eq(18, 10, "SALES"))
//!     .select(vec![(0, 8, 0), (28, 8, 8)])  // Name and salary
//!     .collect();
//!
//! assert_eq!(result.len(), 2);
//! ```

use crate::Record;

/// A pipeline for processing records.
///
/// Pipelines are built using a fluent API and are lazy - no processing
/// occurs until the pipeline is consumed (e.g., via `collect()`).
pub struct Pipeline<I>
where
    I: Iterator<Item = Record>,
{
    iter: I,
}

impl<I> Pipeline<I>
where
    I: Iterator<Item = Record>,
{
    /// Creates a new pipeline from an iterator of records.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![Record::from_str("TEST")];
    /// let pipeline = Pipeline::new(records.into_iter());
    /// ```
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    /// Filters records using a predicate.
    ///
    /// Records for which the predicate returns `false` are removed from
    /// the pipeline. This is similar to DFSORT's INCLUDE operation.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     "),
    ///     Record::from_str("JONES   MARY      ENGINEERING"),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .filter(|r| r.field_eq(18, 10, "SALES"))
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 1);
    /// ```
    pub fn filter<F>(self, predicate: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(&Record) -> bool,
    {
        Pipeline {
            iter: self.iter.filter(predicate),
        }
    }

    /// Omits records matching a predicate.
    ///
    /// Records for which the predicate returns `true` are removed.
    /// This is similar to DFSORT's OMIT operation.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     "),
    ///     Record::from_str("JONES   MARY      ENGINEERING"),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .omit(|r| r.field_eq(18, 10, "SALES"))
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 1);
    /// assert!(result[0].field_eq(0, 8, "JONES"));
    /// ```
    pub fn omit<F>(self, mut predicate: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(&Record) -> bool,
    {
        Pipeline {
            iter: self.iter.filter(move |r| !predicate(r)),
        }
    }

    /// Transforms each record using a function.
    ///
    /// Similar to DFSORT's OUTREC operation.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![Record::from_str("smith   john      ")];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .map(|r| Record::from_str(&r.as_str().to_uppercase()))
    ///     .collect();
    ///
    /// assert!(result[0].as_str().starts_with("SMITH"));
    /// ```
    pub fn map<F>(self, transform: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(Record) -> Record,
    {
        Pipeline {
            iter: self.iter.map(transform),
        }
    }

    /// Transforms records with the option to filter.
    ///
    /// Records for which the transform returns `None` are removed.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     00050000"),
    ///     Record::from_str("JONES   MARY      ENGINEERING00000000"),
    /// ];
    ///
    /// // Keep only records with non-zero salary
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .filter_map(|r| {
    ///         let salary = r.field(29, 8).trim();
    ///         if salary != "00000000" {
    ///             Some(r)
    ///         } else {
    ///             None
    ///         }
    ///     })
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 1);
    /// ```
    pub fn filter_map<F>(self, transform: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(Record) -> Option<Record>,
    {
        Pipeline {
            iter: self.iter.filter_map(transform),
        }
    }

    /// Selects specific fields from records.
    ///
    /// Creates new records containing only the specified fields.
    /// Fields are specified as (source_start, length, dest_start) tuples.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     00050000"),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .select(vec![
    ///         (0, 8, 0),    // Last name -> 0
    ///         (28, 8, 8),   // Salary -> 8
    ///     ])
    ///     .collect();
    ///
    /// assert_eq!(result[0].field(0, 8).trim(), "SMITH");
    /// assert_eq!(result[0].field(8, 8), "00050000");
    /// ```
    pub fn select(
        self,
        fields: Vec<(usize, usize, usize)>,
    ) -> Pipeline<impl Iterator<Item = Record>> {
        Pipeline {
            iter: self.iter.map(move |record| {
                let mut output = Record::new();
                for &(src_start, length, dest_start) in &fields {
                    let value = record.field(src_start, length);
                    output.set_field(dest_start, length, value);
                }
                output
            }),
        }
    }

    /// Reformats records by rearranging fields.
    ///
    /// A convenience wrapper around `map` for field rearrangement.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     "),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .reformat(|r| {
    ///         let mut out = Record::new();
    ///         out.set_field(0, 10, r.field(8, 10));  // First name first
    ///         out.set_field(10, 8, r.field(0, 8));   // Last name second
    ///         out
    ///     })
    ///     .collect();
    ///
    /// assert_eq!(result[0].field(0, 10).trim(), "JOHN");
    /// ```
    pub fn reformat<F>(self, transform: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(&Record) -> Record,
    {
        let mut transform = transform;
        Pipeline {
            iter: self.iter.map(move |r| transform(&r)),
        }
    }

    /// Inspects each record without modifying it.
    ///
    /// Useful for debugging or logging.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![Record::from_str("TEST")];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .inspect(|r| println!("Processing: {}", r.field(0, 8).trim()))
    ///     .collect();
    /// ```
    pub fn inspect<F>(self, callback: F) -> Pipeline<impl Iterator<Item = Record>>
    where
        F: FnMut(&Record),
    {
        Pipeline {
            iter: self.iter.inspect(callback),
        }
    }

    /// Takes the first n records.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("ONE"),
    ///     Record::from_str("TWO"),
    ///     Record::from_str("THREE"),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .take(2)
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 2);
    /// ```
    pub fn take(self, n: usize) -> Pipeline<impl Iterator<Item = Record>> {
        Pipeline {
            iter: self.iter.take(n),
        }
    }

    /// Skips the first n records.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("ONE"),
    ///     Record::from_str("TWO"),
    ///     Record::from_str("THREE"),
    /// ];
    ///
    /// let result: Vec<_> = Pipeline::new(records.into_iter())
    ///     .skip(1)
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 2);
    /// assert!(result[0].as_str().starts_with("TWO"));
    /// ```
    pub fn skip(self, n: usize) -> Pipeline<impl Iterator<Item = Record>> {
        Pipeline {
            iter: self.iter.skip(n),
        }
    }

    /// Chains another iterator of records.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records1 = vec![Record::from_str("ONE")];
    /// let records2 = vec![Record::from_str("TWO")];
    ///
    /// let result: Vec<_> = Pipeline::new(records1.into_iter())
    ///     .chain(records2.into_iter())
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 2);
    /// ```
    pub fn chain<J>(self, other: J) -> Pipeline<impl Iterator<Item = Record>>
    where
        J: Iterator<Item = Record>,
    {
        Pipeline {
            iter: self.iter.chain(other),
        }
    }

    /// Counts the number of records.
    ///
    /// Consumes the pipeline.
    pub fn count(self) -> usize {
        self.iter.count()
    }

    /// Collects all records into a vector.
    ///
    /// Consumes the pipeline.
    pub fn collect(self) -> Vec<Record> {
        self.iter.collect()
    }

    /// Processes all records, discarding the results.
    ///
    /// Useful when the pipeline has side effects (via `inspect`).
    pub fn run(self) {
        for _ in self.iter {}
    }

    /// Returns the first record, if any.
    pub fn first(mut self) -> Option<Record> {
        self.iter.next()
    }

    /// Returns the last record, if any.
    ///
    /// Consumes the entire pipeline.
    pub fn last(self) -> Option<Record> {
        self.iter.last()
    }

    /// Folds records into an accumulator.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::{Pipeline, Record};
    ///
    /// let records = vec![
    ///     Record::from_str("SMITH   JOHN      SALES     00050000"),
    ///     Record::from_str("DOE     JANE      SALES     00060000"),
    /// ];
    ///
    /// let total: u64 = Pipeline::new(records.into_iter())
    ///     .fold(0u64, |acc, r| {
    ///         let salary: u64 = r.field(29, 8).trim().parse().unwrap_or(0);
    ///         acc + salary
    ///     });
    ///
    /// assert_eq!(total, 110000);
    /// ```
    pub fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Record) -> B,
    {
        self.iter.fold(init, f)
    }

    /// Checks if any record matches a predicate.
    pub fn any<F>(mut self, mut predicate: F) -> bool
    where
        F: FnMut(&Record) -> bool,
    {
        self.iter.any(|r| predicate(&r))
    }

    /// Checks if all records match a predicate.
    pub fn all<F>(mut self, mut predicate: F) -> bool
    where
        F: FnMut(&Record) -> bool,
    {
        self.iter.all(|r| predicate(&r))
    }
}

impl<I> Iterator for Pipeline<I>
where
    I: Iterator<Item = Record>,
{
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Creates a pipeline from a slice of strings.
///
/// Convenience function for creating test data.
///
/// # Example
///
/// ```
/// use pipelines_rs::pipeline::from_strings;
///
/// let records: Vec<_> = from_strings(&[
///     "SMITH   JOHN      SALES     ",
///     "JONES   MARY      ENGINEERING",
/// ]).collect();
///
/// assert_eq!(records.len(), 2);
/// ```
pub fn from_strings<'a>(strings: &'a [&'a str]) -> Pipeline<impl Iterator<Item = Record> + 'a> {
    Pipeline::new(strings.iter().map(|s| Record::from_str(s)))
}

/// Creates a pipeline from lines (trimming newlines).
///
/// Useful for reading from files or stdin.
pub fn from_lines<I, S>(lines: I) -> Pipeline<impl Iterator<Item = Record>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    Pipeline::new(lines.map(|s| Record::from_str(s.as_ref())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_records() -> Vec<Record> {
        // Layout: Last(8) First(10) Dept(10) Salary(8)
        vec![
            Record::from_str("SMITH   JOHN      SALES     00050000"),
            Record::from_str("JONES   MARY      ENGINEER  00075000"),
            Record::from_str("DOE     JANE      SALES     00060000"),
            Record::from_str("WILSON  BOB       MARKETING 00055000"),
        ]
    }

    #[test]
    fn test_filter() {
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .filter(|r| r.field_eq(18, 10, "SALES"))
            .collect();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_omit() {
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .omit(|r| r.field_eq(18, 10, "SALES"))
            .collect();

        assert_eq!(result.len(), 2);
        assert!(result[0].field_eq(0, 8, "JONES"));
    }

    #[test]
    fn test_map() {
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .map(|r| Record::from_str(&r.as_str().to_uppercase()))
            .collect();

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_select() {
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .select(vec![(0, 8, 0), (28, 8, 8)])
            .collect();

        assert_eq!(result[0].field(0, 8).trim(), "SMITH");
        assert_eq!(result[0].field(8, 8), "00050000");
    }

    #[test]
    fn test_chain() {
        let records1 = vec![Record::from_str("ONE")];
        let records2 = vec![Record::from_str("TWO")];

        let result: Vec<_> = Pipeline::new(records1.into_iter())
            .chain(records2.into_iter())
            .collect();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_take_skip() {
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .skip(1)
            .take(2)
            .collect();

        assert_eq!(result.len(), 2);
        assert!(result[0].field_eq(0, 8, "JONES"));
        assert!(result[1].field_eq(0, 8, "DOE"));
    }

    #[test]
    fn test_count() {
        let count = Pipeline::new(sample_records().into_iter())
            .filter(|r| r.field_eq(18, 10, "SALES"))
            .count();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_fold() {
        let total: u64 = Pipeline::new(sample_records().into_iter()).fold(0u64, |acc, r| {
            let salary: u64 = r.field(28, 8).trim().parse().unwrap_or(0);
            acc + salary
        });

        assert_eq!(total, 240000); // 50000 + 75000 + 60000 + 55000
    }

    #[test]
    fn test_any_all() {
        let has_sales =
            Pipeline::new(sample_records().into_iter()).any(|r| r.field_eq(18, 10, "SALES"));
        assert!(has_sales);

        let all_sales =
            Pipeline::new(sample_records().into_iter()).all(|r| r.field_eq(18, 10, "SALES"));
        assert!(!all_sales);
    }

    #[test]
    fn test_first_last() {
        let first = Pipeline::new(sample_records().into_iter()).first();
        assert!(first.unwrap().field_eq(0, 8, "SMITH"));

        let last = Pipeline::new(sample_records().into_iter()).last();
        assert!(last.unwrap().field_eq(0, 8, "WILSON"));
    }

    #[test]
    fn test_pipeline_chaining() {
        // Complex pipeline: filter SALES, select name+salary, take first 1
        let result: Vec<_> = Pipeline::new(sample_records().into_iter())
            .filter(|r| r.field_eq(18, 10, "SALES"))
            .select(vec![(0, 8, 0), (28, 8, 8)])
            .take(1)
            .collect();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].field(0, 8).trim(), "SMITH");
        assert_eq!(result[0].field(8, 8), "00050000");
    }

    #[test]
    fn test_from_strings() {
        let result: Vec<_> = from_strings(&["ONE", "TWO", "THREE"]).collect();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_as_iterator() {
        let mut pipeline = Pipeline::new(sample_records().into_iter());

        let first = pipeline.next();
        assert!(first.is_some());
        assert!(first.unwrap().field_eq(0, 8, "SMITH"));

        let count = pipeline.count();
        assert_eq!(count, 3); // Remaining 3 records
    }
}
