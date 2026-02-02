//! Fixed-width 80-byte record type.
//!
//! The 80-byte record width matches the historical punch card format used
//! on mainframe systems. Each record is exactly 80 bytes, padded with spaces
//! if the source data is shorter.

use std::fmt;

/// The standard record width (punch card width).
pub const RECORD_WIDTH: usize = 80;

/// A fixed-width 80-byte record.
///
/// This type represents a single record in mainframe-style batch processing.
/// Records are always exactly 80 bytes, matching the width of punch cards.
///
/// # Field Access
///
/// Fields are accessed by position (0-indexed) and length:
///
/// ```
/// use pipelines_rs::Record;
///
/// let record = Record::from_str("SMITH   JOHN      SALES     ");
/// assert_eq!(record.field(0, 8).trim(), "SMITH");
/// assert_eq!(record.field(8, 10).trim(), "JOHN");
/// assert_eq!(record.field(18, 10).trim(), "SALES");
/// ```
///
/// # Layout Convention
///
/// Typical mainframe record layouts used fixed column positions:
/// - Columns 1-8: Last name (positions 0-7)
/// - Columns 9-18: First name (positions 8-17)
/// - Columns 19-28: Department (positions 18-27)
/// - Columns 29-36: Salary (positions 28-35)
/// - etc.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Record {
    data: [u8; RECORD_WIDTH],
}

impl Record {
    /// Creates a new record filled with spaces.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let record = Record::new();
    /// assert_eq!(record.as_str().len(), 80);
    /// assert!(record.as_str().chars().all(|c| c == ' '));
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: [b' '; RECORD_WIDTH],
        }
    }

    /// Creates a record from a string slice.
    ///
    /// The string is truncated to 80 bytes or padded with spaces if shorter.
    /// Only ASCII characters are supported; non-ASCII bytes are replaced with '?'.
    ///
    /// Note: This method is named `from_str` for convenience but does not
    /// implement `std::str::FromStr` because record parsing never fails.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let record = Record::from_str("Hello, World!");
    /// assert!(record.as_str().starts_with("Hello, World!"));
    /// assert_eq!(record.as_str().len(), 80);
    /// ```
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let mut record = Self::new();
        let bytes = s.as_bytes();
        let len = bytes.len().min(RECORD_WIDTH);

        for (i, &byte) in bytes.iter().take(len).enumerate() {
            // Replace non-ASCII with '?' (simulating EBCDIC conversion issues)
            record.data[i] = if byte.is_ascii() { byte } else { b'?' };
        }

        record
    }

    /// Creates a record from raw bytes.
    ///
    /// The bytes are truncated to 80 or padded with spaces if shorter.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let bytes = b"TEST DATA";
    /// let record = Record::from_bytes(bytes);
    /// assert!(record.as_str().starts_with("TEST DATA"));
    /// ```
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut record = Self::new();
        let len = bytes.len().min(RECORD_WIDTH);

        for (i, &byte) in bytes.iter().take(len).enumerate() {
            record.data[i] = if byte.is_ascii() { byte } else { b'?' };
        }

        record
    }

    /// Returns the record data as a string slice.
    ///
    /// Since we ensure only ASCII bytes are stored, this is always valid UTF-8.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: We only store ASCII bytes, which are valid UTF-8
        // Fallback should never happen, but be safe
        std::str::from_utf8(&self.data).unwrap_or(
            "????????????????????????????????????????????????????????????????????????????????",
        )
    }

    /// Returns the raw bytes of the record.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; RECORD_WIDTH] {
        &self.data
    }

    /// Extracts a field from the record.
    ///
    /// Fields are specified by starting position (0-indexed) and length.
    /// If the field extends beyond the record, it is truncated.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let record = Record::from_str("SMITH   JOHN      ENGINEERING");
    /// assert_eq!(record.field(0, 8), "SMITH   ");
    /// assert_eq!(record.field(8, 10), "JOHN      ");
    /// ```
    #[must_use]
    pub fn field(&self, start: usize, length: usize) -> &str {
        let end = (start + length).min(RECORD_WIDTH);
        let start = start.min(RECORD_WIDTH);

        if start >= end {
            return "";
        }

        std::str::from_utf8(&self.data[start..end]).unwrap_or("")
    }

    /// Sets a field in the record.
    ///
    /// The value is truncated if longer than the field length, or padded
    /// with spaces if shorter.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let mut record = Record::new();
    /// record.set_field(0, 8, "SMITH");
    /// record.set_field(8, 10, "JOHN");
    /// assert_eq!(record.field(0, 8), "SMITH   ");
    /// assert_eq!(record.field(8, 10), "JOHN      ");
    /// ```
    pub fn set_field(&mut self, start: usize, length: usize, value: &str) {
        let end = (start + length).min(RECORD_WIDTH);
        let start = start.min(RECORD_WIDTH);

        if start >= end {
            return;
        }

        // Clear the field with spaces first
        for byte in &mut self.data[start..end] {
            *byte = b' ';
        }

        // Copy the value
        let value_bytes = value.as_bytes();
        let copy_len = value_bytes.len().min(end - start);

        for (i, &byte) in value_bytes.iter().take(copy_len).enumerate() {
            self.data[start + i] = if byte.is_ascii() { byte } else { b'?' };
        }
    }

    /// Returns true if the record is blank (all spaces).
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.data.iter().all(|&b| b == b' ')
    }

    /// Compares a field to a value.
    ///
    /// This is a convenience method for filtering operations.
    ///
    /// # Example
    ///
    /// ```
    /// use pipelines_rs::Record;
    ///
    /// let record = Record::from_str("SMITH   JOHN      SALES     ");
    /// assert!(record.field_eq(18, 10, "SALES"));
    /// assert!(!record.field_eq(18, 10, "ENGINEERING"));
    /// ```
    #[must_use]
    pub fn field_eq(&self, start: usize, length: usize, value: &str) -> bool {
        self.field(start, length).trim() == value.trim()
    }

    /// Compares a field to a value with exact matching (including spaces).
    #[must_use]
    pub fn field_eq_exact(&self, start: usize, length: usize, value: &str) -> bool {
        self.field(start, length) == value
    }

    /// Returns true if a field starts with the given prefix.
    #[must_use]
    pub fn field_starts_with(&self, start: usize, length: usize, prefix: &str) -> bool {
        self.field(start, length).trim_start().starts_with(prefix)
    }

    /// Returns true if a field contains the given substring.
    #[must_use]
    pub fn field_contains(&self, start: usize, length: usize, substring: &str) -> bool {
        self.field(start, length).contains(substring)
    }
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Record({:?})", self.as_str().trim_end())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for Record {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl From<&[u8]> for Record {
    fn from(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_record_is_blank() {
        let record = Record::new();
        assert!(record.is_blank());
        assert_eq!(record.as_str().len(), 80);
    }

    #[test]
    fn test_from_str_short() {
        let record = Record::from_str("HELLO");
        assert!(record.as_str().starts_with("HELLO"));
        assert_eq!(record.as_str().len(), 80);
        assert_eq!(&record.as_str()[5..], &" ".repeat(75));
    }

    #[test]
    fn test_from_str_exact() {
        let input = "A".repeat(80);
        let record = Record::from_str(&input);
        assert_eq!(record.as_str(), input);
    }

    #[test]
    fn test_from_str_truncated() {
        let input = "B".repeat(100);
        let record = Record::from_str(&input);
        assert_eq!(record.as_str(), &"B".repeat(80));
    }

    #[test]
    fn test_field_extraction() {
        let record = Record::from_str("SMITH   JOHN      ENGINEERING00075000");
        assert_eq!(record.field(0, 8), "SMITH   ");
        assert_eq!(record.field(8, 10), "JOHN      ");
        assert_eq!(record.field(18, 11), "ENGINEERING");
        assert_eq!(record.field(29, 8), "00075000");
    }

    #[test]
    fn test_field_eq() {
        let record = Record::from_str("SMITH   JOHN      SALES     ");
        assert!(record.field_eq(0, 8, "SMITH"));
        assert!(record.field_eq(8, 10, "JOHN"));
        assert!(record.field_eq(18, 10, "SALES"));
        assert!(!record.field_eq(18, 10, "ENGINEERING"));
    }

    #[test]
    fn test_set_field() {
        let mut record = Record::new();
        record.set_field(0, 8, "SMITH");
        record.set_field(8, 10, "JOHN");
        record.set_field(18, 10, "SALES");
        record.set_field(28, 8, "00050000");

        assert_eq!(record.field(0, 8).trim(), "SMITH");
        assert_eq!(record.field(8, 10).trim(), "JOHN");
        assert_eq!(record.field(18, 10).trim(), "SALES");
        assert_eq!(record.field(28, 8), "00050000");
    }

    #[test]
    fn test_set_field_truncates() {
        let mut record = Record::new();
        record.set_field(0, 5, "LONGERNAME");
        assert_eq!(record.field(0, 5), "LONGE");
    }

    #[test]
    fn test_non_ascii_replaced() {
        let record = Record::from_str("Hello\u{00E9}World"); // e with acute
        assert!(record.as_str().contains('?'));
    }

    #[test]
    fn test_field_out_of_bounds() {
        let record = Record::from_str("TEST");
        // Should not panic, just return truncated/empty
        assert_eq!(record.field(90, 10), "");
        assert_eq!(record.field(75, 10), "     "); // partial
    }

    #[test]
    fn test_field_contains() {
        let record = Record::from_str("SMITH   JOHN      ENGINEERING");
        assert!(record.field_contains(18, 11, "ENGINE"));
        assert!(!record.field_contains(18, 11, "SALES"));
    }

    #[test]
    fn test_field_starts_with() {
        let record = Record::from_str("SMITH   JOHN      ENGINEERING");
        assert!(record.field_starts_with(18, 11, "ENG"));
        assert!(!record.field_starts_with(18, 11, "SALES"));
    }

    #[test]
    fn test_display() {
        let record = Record::from_str("TEST");
        let displayed = format!("{record}");
        assert_eq!(displayed.len(), 80);
        assert!(displayed.starts_with("TEST"));
    }

    #[test]
    fn test_debug() {
        let record = Record::from_str("TEST   ");
        let debug = format!("{record:?}");
        assert!(debug.contains("TEST"));
        // Debug should trim trailing spaces
        assert!(!debug.ends_with("   \")"));
    }
}
