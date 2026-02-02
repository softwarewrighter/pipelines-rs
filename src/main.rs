//! Demonstration of mainframe-style 80-byte record pipeline processing.
//!
//! This example shows how batch processing worked on mainframe systems,
//! where data was processed as fixed-width 80-byte records (matching
//! the width of punch cards).

use pipelines_rs::{Pipeline, Record};

fn main() {
    println!("=== Mainframe-Style Pipeline Processing Demo ===\n");

    // Sample employee records (80-byte fixed width)
    // Layout:
    //   Columns 1-8  (pos 0-7):   Last Name
    //   Columns 9-18 (pos 8-17):  First Name
    //   Columns 19-28 (pos 18-27): Department
    //   Columns 29-36 (pos 28-35): Employee ID
    //   Columns 37-44 (pos 36-43): Salary
    //   Columns 45-52 (pos 44-51): Hire Date
    //   Columns 53-80 (pos 52-79): Filler/Reserved

    // Fixed-width record layout (80 bytes total):
    // Positions: 0         1         2         3         4         5         6         7
    //            01234567890123456789012345678901234567890123456789012345678901234567890123456789
    // Fields:    LASTNAME FIRSTNAME  DEPARTMENT EMP_ID  SALARY  HIREDATE  FILLER
    //            |8 chars||10 chars ||10 chars ||8chars||8chars ||8 chars |
    let employee_records = vec![
        Record::from_str("SMITH   JOHN      SALES     EMP001230005000019850315                "),
        Record::from_str("JONES   MARY      ENGINEER  EMP004560007500019900622                "),
        Record::from_str("DOE     JANE      SALES     EMP007890006000019880101                "),
        Record::from_str("WILSON  ROBERT    MARKETING EMP002340005500019920815                "),
        Record::from_str("CHEN    LISA      ENGINEER  EMP005670008000019950303                "),
        Record::from_str("GARCIA  CARLOS    SALES     EMP008900004500019870720                "),
        Record::from_str("TAYLOR  SUSAN     MARKETING EMP003450006500019910112                "),
        Record::from_str("BROWN   MICHAEL   ENGINEER  EMP006780009000019980505                "),
    ];

    println!("Input Records ({} total):", employee_records.len());
    println!("{}", "-".repeat(80));
    for record in &employee_records {
        println!("{}", record);
    }
    println!("{}\n", "-".repeat(80));

    // Pipeline 1: Filter SALES department
    println!("=== Pipeline 1: INCLUDE DEPT='SALES' ===");
    let sales_only: Vec<_> = Pipeline::new(employee_records.clone().into_iter())
        .filter(|r| r.field_eq(18, 10, "SALES"))
        .collect();

    println!("Output ({} records):", sales_only.len());
    for record in &sales_only {
        println!(
            "  {} {} - ${}",
            record.field(0, 8).trim(),
            record.field(8, 10).trim(),
            record.field(36, 8).trim()
        );
    }
    println!();

    // Pipeline 2: Select specific fields (like DFSORT OUTREC)
    println!("=== Pipeline 2: SELECT NAME, SALARY ===");
    let name_salary: Vec<_> = Pipeline::new(employee_records.clone().into_iter())
        .select(vec![
            (0, 8, 0),   // Last name -> col 1
            (8, 10, 8),  // First name -> col 9
            (36, 8, 18), // Salary -> col 19
        ])
        .collect();

    println!("Output ({} records):", name_salary.len());
    for record in &name_salary {
        println!(
            "  {} {} ${}",
            record.field(0, 8).trim(),
            record.field(8, 10).trim(),
            record.field(18, 8).trim()
        );
    }
    println!();

    // Pipeline 3: Filter and reformat
    println!("=== Pipeline 3: ENGINEER DEPT, REFORMATTED ===");
    let eng_formatted: Vec<_> = Pipeline::new(employee_records.clone().into_iter())
        .filter(|r| r.field_eq(18, 10, "ENGINEER"))
        .reformat(|r| {
            let mut out = Record::new();
            // Format: "FIRSTNAME LASTNAME (EMPID) - $SALARY"
            out.set_field(0, 10, r.field(8, 10)); // First name
            out.set_field(10, 8, r.field(0, 8)); // Last name
            out.set_field(18, 10, r.field(28, 8)); // Emp ID
            out.set_field(28, 8, r.field(36, 8)); // Salary
            out
        })
        .collect();

    println!("Output ({} records):", eng_formatted.len());
    for record in &eng_formatted {
        println!(
            "  {} {} ({}) - ${}",
            record.field(0, 10).trim(),
            record.field(10, 8).trim(),
            record.field(18, 10).trim(),
            record.field(28, 8).trim()
        );
    }
    println!();

    // Pipeline 4: Calculate total salary
    println!("=== Pipeline 4: SUM SALARY ===");
    let total_salary: u64 =
        Pipeline::new(employee_records.clone().into_iter()).fold(0u64, |acc, r| {
            let salary: u64 = r.field(36, 8).trim().parse().unwrap_or(0);
            acc + salary
        });

    println!("Total Salary: ${total_salary}");
    println!();

    // Pipeline 5: Chain multiple sources (like MERGE without sort)
    println!("=== Pipeline 5: CHAIN TWO DEPARTMENTS ===");
    let sales =
        Pipeline::new(employee_records.clone().into_iter()).filter(|r| r.field_eq(18, 10, "SALES"));

    let marketing: Vec<_> = Pipeline::new(employee_records.clone().into_iter())
        .filter(|r| r.field_eq(18, 10, "MARKETING"))
        .collect();

    let combined: Vec<_> = sales.chain(marketing.into_iter()).collect();

    println!("Output ({} records - SALES + MARKETING):", combined.len());
    for record in &combined {
        println!(
            "  {} - {}",
            record.field(0, 8).trim(),
            record.field(18, 10).trim()
        );
    }
    println!();

    // Pipeline 6: Statistics
    println!("=== Pipeline 6: DEPARTMENT STATISTICS ===");
    let departments = ["SALES", "ENGINEER", "MARKETING"];

    for dept in departments {
        let (count, total): (usize, u64) = employee_records
            .iter()
            .filter(|r| r.field_eq(18, 10, dept))
            .fold((0, 0), |(count, total), r| {
                let salary: u64 = r.field(36, 8).trim().parse().unwrap_or(0);
                (count + 1, total + salary)
            });

        let avg = if count > 0 { total / count as u64 } else { 0 };
        println!("  {dept:<12} Count: {count}, Total: ${total}, Avg: ${avg}");
    }
    println!();

    println!("=== Demo Complete ===");
}
