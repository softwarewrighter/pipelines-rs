# Multi-Stage Pipeline Specifications Design

## Overview

This document describes design and implementation plan for adding multi-stage pipeline specifications to pipelines-rs. This feature enables defining and running multiple interconnected pipelines in a single specification file, following CMS Pipelines pattern of using `?` as a pipeline separator.

## Design Philosophy: Unix/Linux Record-Oriented Pipelines

**Goal:** Provide a clean-room feature-subset of CMS Pipelines adapted for Unix/Linux/Mac environments.

**Key Principles:**
1. **Record-oriented, not stream-oriented**: Process 80-byte fixed-width ASCII records (columnar data)
2. **ASCII-only**: No EBCDIC support (we're on Linux/Mac, not mainframe)
3. **Unix pipe equivalents**: Provide stages with familiar Unix functionality adapted for record-based processing
4. **Columnar data focus**: Data is structured in fixed columns, not CSV, not JSON, not binary
5. **Practical subset**: Focus on useful stages for real data processing tasks, not mainframe-specific features

**Avoid Mainframe-Specific Features:**
- VMARC/VMREAD/VMWRITE formats
- BOOK documentation formats
- LIST3820 listing formats
- EBCDIC translation stages
- IUCV inter-VM communication
- CP/CMS command stages
- Mainframe-specific file handling

**Include Unix-Adapted Features:**
- File I/O (read/write text files)
- Column-based filtering and selection
- Field extraction and reformatting
- Sorting and ordering
- Joining records (like Unix `join`)
- Aggregation (sum, avg, min, max, count)
- Text transformation (search/replace, case changes)
- Splitting records (like Unix `split`)
- Merging sorted streams
- Uniqueness filtering
- Pagination (windowing)

## Background: CMS Pipelines Multi-Stage Syntax

CMS Pipelines supports "pipeline sets" - multiple pipelines that work together. The key concepts:

1. **Pipeline Separator**: The `?` character separates individual pipelines in a pipeline set
2. **Labels**: Stages can be labeled (e.g., `a:`) for connecting streams between pipelines
3. **Stream Splitting**: Stages like `locate` can split output into primary and secondary streams
4. **Stream Merging**: Stages like `faninany` combine multiple input streams into one

Example from CMS Pipelines documentation:
```
PIPE (end ?)
          < input txt
        | a: locate /Hello/
        | insert / World!/ after
        | i: faninany
        | > newfile txt a
 ? a:
        | xlate upper
        | i:
```

## Proposed Syntax for Unix/Linux Record-Oriented Pipelines

### Single Pipeline (Current - Already Supported)

```
PIPE CONSOLE
| FILTER 18,10 = "SALES"
| CONSOLE
?
```

### Multiple Pipelines (New - Chained)

Output of one pipeline automatically feeds into the next:

```
# Pipeline 1: Filter for SALES
PIPE CONSOLE
| LOCATE /SALES/
?
# Pipeline 2: Select name and salary fields
| SELECT 0,8,0; 28,8,8
?
# Pipeline 3: Output
| CONSOLE
?
```

**Key distinction:**
- Pipelines end with `?` separator
- Last pipeline's output goes to actual console
- Intermediate pipelines' output becomes next pipeline's input
- No need for explicit file I/O between stages

### File I/O Stages

**FILE source stage (read from file):**
```
PIPE FILE input.txt
| FILTER 18,10 = "SALES"
| CONSOLE
?
```

**File sink stage (write to file):**
```
PIPE CONSOLE
| FILTER 18,10 = "SALES"
| > output.txt
?
```

**Independent pipelines with file I/O:**
```
# Pipeline 1: Extract and save to intermediate file
PIPE CONSOLE
| LOCATE /SALES/
| > work/sales-intermediate.txt
?

# Pipeline 2: Process from intermediate file
PIPE FILE work/sales-intermediate.txt
| SELECT 0,8,0; 28,8,8
| CONSOLE
?
```

## Unix-Style Pipeline Stages (Planned)

### Sorting (SORT)

Equivalent to Unix `sort` but for fixed-width records:

```
PIPE CONSOLE
| SORT 0,10,asc
| CONSOLE
?
```

**Syntax:**
- `SORT pos,len,direction` - Sort by field at position
- `direction` can be `asc` (ascending) or `desc` (descending)

**Example use cases:**
- Sort employee records by name (ascending)
- Sort by salary (descending)
- Sort by date (oldest/newest)

### Joining (JOIN)

Join records from two fields like Unix `join`:

```
PIPE CONSOLE
| JOIN left_file.txt right_field
| CONSOLE
?
```

**Note:** This is more complex - defer to Phase 6 or implement simpler variant

### Splitting (SPLIT)

Split records by delimiter like Unix `split`:

```
PIPE CONSOLE
| SPLIT /, 5
| CONSOLE
?
```

**Syntax:** `SPLIT delimiter field_num`
- Split each record at delimiter
- Keep specified field (1-based or 0-based)

### Uniqueness (UNIQ)

Remove duplicate records like Unix `uniq`:

```
PIPE CONSOLE
| UNIQ
| CONSOLE
?
```

**Note:** Consider field-based uniqueness vs whole-record uniqueness

### Aggregation (SUM, AVG, MIN, MAX, COUNT)

Aggregate numeric fields across records:

```
PIPE CONSOLE
| SUM 28,8
| CONSOLE
?
```

**Syntax:**
- `SUM pos,len` - Sum numeric field
- `AVG pos,len` - Average numeric field
- `MIN pos,len` - Find minimum value
- `MAX pos,len` - Find maximum value

## Implementation Plan

### Phase 1: Core Multi-Pipeline Parser

#### 1.1 Modify DSL Parser (`src/dsl.rs`)

**Current structure:**
```rust
pub fn execute_pipeline(
    input_text: &str,
    pipeline_text: &str,
) -> Result<(String, usize, usize), String>
```

**New structure:**
```rust
pub fn execute_multi_pipeline(
    input_text: &str,
    pipeline_text: &str,
) -> Result<(String, usize, usize), String>
```

**Data structures:**
```rust
/// Represents a single pipeline in a multi-pipeline spec
struct SinglePipeline {
    /// Source stage (CONSOLE, FILE, LITERAL, or HOLE)
    source: Command,
    /// Transform stages (middle stages)
    transforms: Vec<Command>,
    /// Sink stage (CONSOLE, FILE, or HOLE)
    sink: Command,
}

/// Parsed multi-pipeline specification
struct MultiPipelineSpec {
    /// All pipelines in sequence
    pipelines: Vec<SinglePipeline>,
}
```

**Parsing logic:**
1. Split input by `?` markers (after trimming whitespace)
2. For each pipeline segment, parse as a complete pipeline (source + transforms + sink)
3. Validate that each pipeline has at least a source and sink
4. Return `MultiPipelineSpec`

#### 1.2 Execution Model

**Option 1: Sequential Execution**
```rust
pub fn execute_multi_pipeline(
    input_text: &str,
    pipeline_text: &str,
) -> Result<(String, usize, usize), String> {
    let spec = parse_multi_pipeline(pipeline_text)?;

    let mut current_input = input_text.to_string();
    let mut total_input_count = 0;
    let mut total_output_count = 0;

    for pipeline in &spec.pipelines {
        // Execute this pipeline
        let (output, input_count, output_count) =
            execute_single_pipeline(&current_input, pipeline)?;

        total_input_count += input_count;
        total_output_count += output_count;

        // Check if pipeline writes to file (ends with FILE sink)
        if matches!(pipeline.sink, Command::FileSink { .. }) {
            // Output is written to file, don't chain
            // Reset for next independent pipeline
            current_input = input_text.to_string();
        } else {
            // Output becomes input for next pipeline (chaining)
            current_input = output;
        }
    }

    // Final output from last pipeline
    Ok((current_input, total_input_count, total_output_count))
}
```

**Option 2: Streaming Execution (Future)**
For better memory efficiency, stream records between pipelines instead of collecting all into strings:
```rust
pub fn execute_multi_pipeline_stream<'a>(
    input_records: impl Iterator<Item = Record> + 'a,
    pipeline_text: &str,
) -> Result<Box<dyn Iterator<Item = Record> + 'a>, String>
```

**Recommendation:** Start with Option 1 (simpler), optimize to Option 2 later if needed.

### Phase 2: File I/O Stages

#### 2.1 Add FILE Source/Sink Stages

**Syntax:**
```
PIPE FILE input.txt
| FILTER 18,10 = "SALES"
| CONSOLE
?
```

**Implementation:**
```rust
enum Command {
    // ... existing ...
    
    /// Read from file (source)
    FileSource { path: String },
    
    /// Write to file (sink)
    FileSink { path: String },
}
```

**Parser updates:**
```rust
fn parse_command(line: &str) -> Result<Command, String> {
    let upper = line.to_uppercase();

    if upper.starts_with("FILE ") {
        let rest = line[5..].trim();
        Ok(Command::FileSource { path: rest.to_string() })
    } else if upper.starts_with("> ") {
        let rest = line[2..].trim();  // Skip "> " or '>'
        Ok(Command::FileSink { path: rest.to_string() })
    }
    // ... existing ...
}
```

**CLI integration:**
- File paths in pipeline specs are relative to spec file location
- Add working directory context to `execute_multi_pipeline`

**Working directory handling:**
```rust
fn resolve_pipeline_path(spec_file: &Path, pipeline_path: &str) -> PathBuf {
    if pipeline_path.starts_with('/') {
        // Absolute path - use as-is
        PathBuf::from(pipeline_path)
    } else {
        // Relative path - resolve relative to spec file
        spec_file
            .parent()
            .unwrap_or(Path::new("."))
            .join(pipeline_path)
    }
}

fn ensure_output_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create output directory '{}': {}", parent.display(), e))?;
        }
    }
    Ok(())
}
```

### Phase 3: Unix-Style Stages (Basic Set)

#### 3.1 SORT Stage

Sort records by numeric or string fields:

```rust
enum Command {
    // ... existing ...
    
    /// Sort by field
    Sort { pos: usize, len: usize, direction: SortDirection },
}

#[derive(Clone, Copy)]
enum SortDirection {
    Asc,
    Desc,
}

fn parse_sort(line: &str) -> Result<Command, String> {
    let rest = line[4..].trim();
    
    // Parse: "pos,len,direction"
    let parts: Vec<&str> = rest.split(',').collect();
    
    if parts.len() != 3 {
        return Err("SORT requires pos,len,direction".to_string());
    }
    
    let pos: usize = parts[0].trim()
        .parse()
        .map_err(|_| "Invalid position number")?;
    let len: usize = parts[1].trim()
        .parse()
        .map_err(|_| "Invalid length number")?;
    
    let direction = match parts[2].trim().to_uppercase().as_str() {
        "ASC" => SortDirection::Asc,
        "DESC" => SortDirection::Desc,
        _ => return Err("SORT direction must be ASC or DESC".to_string()),
    };
    
    Ok(Command::Sort { pos, len, direction })
}

fn apply_sort(records: Vec<Record>, cmd: &Command::Sort) -> Result<Vec<Record>, String> {
    let pos = cmd.pos;
    let len = cmd.len;
    let direction = cmd.direction;
    
    let mut records = records;
    
    records.sort_by(|a, b| {
        let field_a = a.field(pos, len).trim();
        let field_b = b.field(pos, len).trim();
        
        // Try numeric comparison first
        let field_a_num = field_a.parse::<f64>();
        let field_b_num = field_b.parse::<f64>();
        
        match (field_a_num, field_b_num) {
            (Some(na), Some(nb)) => match direction {
                SortDirection::Asc => na.partial_cmp(&nb).unwrap(),
                SortDirection::Desc => nb.partial_cmp(&na).unwrap(),
            },
            _ => match direction {
                SortDirection::Asc => field_a.cmp(field_b),
                SortDirection::Desc => field_b.cmp(field_a),
            },
        }
    });
    
    Ok(records)
}
```

#### 3.2 SPLIT Stage

Split records at delimiter:

```rust
enum Command {
    // ... existing ...
    
    /// Split records at delimiter
    Split { delimiter: String, keep_field: Option<usize> },
}

fn parse_split(line: &str) -> Result<Command, String> {
    let rest = line[5..].trim();
    
    // Format: "SPLIT delimiter [keep_field_num]"
    let parts: Vec<&str> = rest.split_whitespace().collect();
    
    if parts.is_empty() || parts.len() > 2 {
        return Err("SPLIT requires delimiter and optional keep_field".to_string());
    }
    
    let delimiter = parts[0].to_string();
    let keep_field = if parts.len() == 2 {
        Some(parts[1].trim().parse().map_err(|_| "Invalid field number")?)
    } else {
        None
    };
    
    Ok(Command::Split { delimiter, keep_field })
}

fn apply_split(records: Vec<Record>, cmd: &Command::Split) -> Result<Vec<Record>, String> {
    let delimiter = &cmd.delimiter;
    let keep_field = cmd.keep_field;
    
    let result: Vec<Record> = records.iter()
        .map(|rec| {
            let content = rec.as_str();
            
            if let Some(field_num) = keep_field {
                // Split and keep specific field
                let parts: Vec<&str> = content.split(delimiter).collect();
                if let Some(field) = parts.get(field_num.saturating_sub(1)) {
                    Record::from_str(field)
                } else {
                    rec.clone()
                }
            } else {
                // Just split at delimiter, keep all parts
                content.split(delimiter)
                    .map(|part| Record::from_str(part))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .flatten()
                    .collect()
            }
        })
        .collect();
    
    Ok(result)
}
```

#### 3.3 UNIQ Stage

Remove duplicate records:

```rust
enum Command {
    // ... existing ...
    
    /// Remove duplicate records
    Uniq,
}

fn parse_uniq(line: &str) -> Result<Command, String> {
    // Format: "UNIQ [field]"
    let rest = line[4..].trim();
    
    if rest.is_empty() {
        Ok(Command::Uniq)
    } else {
        // Optional field-based uniqueness (deferred - Phase 6)
        Err("UNIQ does not support field specification yet".to_string())
    }
}

fn apply_uniq(records: Vec<Record>) -> Result<Vec<Record>, String> {
    use std::collections::HashSet;
    
    let mut seen = HashSet::new();
    let result: Vec<Record> = records.into_iter()
        .filter(|rec| {
            let content = rec.as_str();
            !content.contains('\n') && seen.insert(content)
        })
        .collect();
    
    Ok(result)
}
```

### Phase 4: CLI Updates

#### 4.1 Update `pipe-run` CLI

**Current usage:**
```bash
pipe-run <pipeline.pipe> <input.data> [-o output.data]
```

**New usage (unchanged - CLI handles multi-pipeline specs transparently):**
```bash
pipe-run <pipeline.pipe> <input.data> [-o output.data]
```

The CLI reads to pipeline file, which may contain multiple `?`-separated pipelines, and executes them.

### Phase 5: WASM UI Updates

#### 5.1 Enhanced Tutorial Menu Structure

Replace flat tutorial dropdown with a hierarchical menu system:

**Tutorial Categories:**
- Single-Stage Tutorials (basic pipeline concepts)
- Multi-Stage Tutorials (chained pipelines)
- Canned Examples (production use cases)

**UI Structure:**
```rust
// In wasm-ui/src/app.rs
#[derive(Clone, PartialEq)]
enum TutorialCategory {
    None,
    SingleStage,    // 15 existing tutorials
    MultiStage,     // New multi-stage tutorials
    Examples,        // Canned production examples
}

#[derive(Clone, PartialEq)]
pub struct TutorialGroup {
    pub category: TutorialCategory,
    pub name: &'static str,
    pub tutorials: Vec<TutorialStep>,
}
```

**Tutorial Groups:**

```rust
const TUTORIALS_SINGLE_STAGE: &[TutorialStep] = &[
    // ... existing 15 single-stage tutorials ...
];

const TUTORIALS_MULTI_STAGE: &[TutorialStep] = &[
    TutorialStep {
        name: "Chained Pipelines",
        description: "Multiple pipelines run in sequence.\n\n\
            The ? character separates pipelines.\n\
            Output of one pipeline becomes input to the next.\n\n\
            This example filters, then selects fields in two steps.",
        example_pipeline: r#"# Pipeline 1: Filter for SALES
PIPE CONSOLE
| LOCATE /SALES/
?
# Pipeline 2: Select name and salary
| SELECT 0,8,0; 28,8,8
?
# Pipeline 3: Output
| CONSOLE
?"#,
    },
    TutorialStep {
        name: "Pipeline with Header",
        description: "Combine LITERAL, filtering, and selection.\n\n\
            Pipeline 1 adds a header line.\n\
            Pipeline 2 filters for SALES.\n\
            Pipeline 3 extracts fields.\n\
            Pipeline 4 adds a footer line.",
        example_pipeline: r#"# Add header
PIPE LITERAL ==================== SALES REPORT ====================
?
# Filter for SALES
| LOCATE /SALES/
?
# Extract name and salary
| SELECT 0,8,0; 28,8,8
?
# Add footer
| LITERAL ==========================================================
?
# Output
| CONSOLE
?"#,
    },
];

const TUTORIALS_EXAMPLES: &[TutorialStep] = &[
    // Canned non-trivial examples for production use cases
    TutorialStep {
        name: "Sales Department Report",
        description: "Complete sales report with header, data, and summary.\n\n\
            This is a real-world example showing:\n\
            - Header row with column headers\n\
            - Filtered and formatted sales data\n\
            - Count summary at the end",
        example_pipeline: r#"# Complex sales report
# Add report header with column labels
PIPE LITERAL NAME       DEPT       SALARY     NOTES
?
# Filter for sales department and format
| LOCATE /SALES/
| SELECT 0,10,0; 18,10,10; 36,8,20
?
# Count records and add summary
| COUNT
| LITERAL Total sales records: 
?
# Format count and output
| SELECT 0,22,0
?
| CONSOLE
?"#,
    },
];
```

#### 5.2 Enhanced LOAD Button for Canned Examples

**Current LOAD button:** Only uploads `.pipe` files from user's filesystem.

**Enhanced LOAD button:** Add dropdown with two options:

1. **"Upload .pipe file"** - Current functionality (hidden file input)
2. **"Load Example"** - Shows submenu with canned examples

**Example Library Structure:**

Create a new Rust file `wasm-ui/src/examples.rs`:

```rust
/// Pre-built pipeline examples with comments and descriptions
pub struct CannedExample {
    pub name: &'static str,
    pub category: ExampleCategory,
    pub description: &'static str,
    pub pipeline: &'static str,
    pub input_data: Option<&'static str>,  // Optional default input
    pub tags: Vec<&'static str>,       // For filtering/searching
}

#[derive(Clone, Copy)]
pub enum ExampleCategory {
    Basic,
    Filtering,
    Transformation,
    MultiStage,
    Reporting,
    Production,
}

pub const EXAMPLES: &[CannedExample] = &[
    CannedExample {
        name: "Filter Sales Department",
        category: ExampleCategory::Filtering,
        description: "Filter records where department field equals SALES (columns 18-27)",
        pipeline: r#"# Filter for SALES department
# Keep only records where department is SALES
PIPE CONSOLE
| FILTER 18,10 = "SALES"
| CONSOLE
?"#,
        input_data: Some(r#"SMITH   JOHN      SALES     00050000
JONES   MARY      ENGINEER  00075000
DOE     JANE      SALES     00060000
WILSON  ROBERT    MARKETING 00055000"#),
        tags: vec!["filter", "sales", "department"],
    },

    // ... many more examples ...
];
```

#### 5.3 Enhanced Input Data Loading

**Current Input Panel:** Textarea with manual entry only.

**Enhanced Input Panel:** Add capability to load `.f80` files (80-byte fixed-width records).

**File Format (.f80):**
- Fixed 80 bytes per line (punch card format)
- Text content, same as `.pipe` files but specialized extension
- UTF-8 encoding
- Lines can be shorter than 80 bytes (right-padded with spaces)

**Implementation:**

```rust
// In wasm-ui/src/components.rs
#[derive(Properties, PartialEq)]
pub struct InputPanelProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub on_load_file: Callback<web_sys::Event>,
    pub on_load_f80: Callback<web_sys::Event>,  // New!
}

#[function_component(InputPanel)]
pub fn input_panel(props: &InputPanelProps) -> Html {
    let on_f80_upload = {
        let props = props.clone();
        Callback::from(move |e: web_sys::Event| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let filename = file.name();
                    
                    // Accept both .f80 and .txt files
                    let is_valid_ext = filename.ends_with(".f80") || filename.ends_with(".txt");
                    
                    if !is_valid_ext {
                        // Show error notification
                        // TODO: Implement toast notification
                        return;
                    }
                    
                    let reader = web_sys::FileReader::new().unwrap();
                    let on_load = {
                        let props = props.clone();
                        Closure::wrap(Box::new(move |event: web_sys::Event| {
                            if let Ok(text) = event
                                .target()
                                .unwrap()
                                .unchecked_into::<web_sys::ProgressEvent>()
                                .result()
                                .as_string()
                            {
                                props.on_change.emit(text);
                            }
                        })
                    };
                    
                    reader.set_onload(Some(&on_load));
                    reader.read_as_text(&file).unwrap();
                }
            }
            
            // Clear input so same file can be loaded again
            input.set_value("");
        })
    };

    html! {
        <div class="input-panel">
            <div class="panel-header">
                <h2>{"Input Records"}</h2>
                <div class="panel-controls">
                    <button
                        class="control-button"
                        onclick={on_f80_upload.clone()}
                        title="Load .f80 or .txt file"
                    >
                        {"📁 Load Data File"}
                    </button>
                    <button
                        class="control-button"
                        onclick={Callback::from(|_| {
                            props.on_change.emit(String::new());
                        })}
                        title="Clear input panel"
                    >
                        {"🗑️ Clear"}
                    </button>
                </div>
            </div>
            <textarea
                class="input-textarea"
                value={props.value.clone()}
                oninput={Callback::from(|e: InputEvent| {
                    props.on_change.emit(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value());
                })}
                placeholder="Enter 80-byte fixed-width records, one per line"
                spellcheck="false"
            />
            <div class="input-ruler">
                {0..8}.each(|i| html! {<span>{i}</span>}).collect::<Html>()
            </div>
        </div>
    }
}
```

**File Type Validation:**

```rust
// Validate uploaded files
fn validate_f80_file(filename: &str, content: &str) -> Result<(), String> {
    // Check file extension
    if !filename.ends_with(".f80") && !filename.ends_with(".txt") {
        return Err("Only .f80 and .txt files are supported".to_string());
    }
    
    // Validate line lengths
    let mut line_num = 1;
    for line in content.lines() {
        if !line.is_empty() && line.len() > 80 {
            return Err(format!("Line {} exceeds 80 bytes (found {})", line_num, line.len()));
        }
        line_num += 1;
    }
    
    Ok(())
}
```

### Phase 6: Advanced Features (Post-MVP)

These features can be added after the initial multi-pipeline support is working:

#### 6.1 Field-Based UNIQ

Make UNIQ stage field-aware:
```
PIPE CONSOLE
| UNIQ 0,8    # Remove duplicates based on name field
| CONSOLE
?
```

#### 6.2 JOIN Stage (Simplified)

Simple join by concatenating records:
```
PIPE CONSOLE
| JOIN 2
| CONSOLE
?
```

**Syntax:** `JOIN field_num`
- Join current record with next record at specified field

#### 6.3 Aggregation Improvements

Support multiple aggregate functions:
- `SUM pos,len` - Sum numeric field
- `AVG pos,len` - Average numeric field
- `MIN pos,len` - Find minimum
- `MAX pos,len` - Find maximum
- `COUNT pos,len` - Count non-empty fields

#### 6.4 Conditional Pipelines

Simple conditional execution:
```
PIPE CONSOLE
| COUNT
| IF > 10 THEN
| TAKE 5
| CONSOLE
?
```

**Syntax:** `IF operator value THEN stage`
- Operators: `=`, `!=`, `>`, `<`, `>=`, `<=`
- Based on COUNT output or field value

## Example Use Cases

### Use Case 1: Unix-Style Data Filtering Pipeline

```pipe
# Filter for SALES, extract key fields
PIPE CONSOLE
| LOCATE /SALES/
| SELECT 0,8,0; 28,8,8
?
# Output
| CONSOLE
?
```

**Use case:** Extract sales data from employee records, showing only name and salary.

### Use Case 2: Data Preparation with Intermediate File

```pipe
# Step 1: Extract and save
PIPE CONSOLE
| LOCATE /SALES/
| > work/sales-raw.txt
?

# Step 2: Read, transform, save
PIPE FILE work/sales-raw.txt
| SELECT 0,8,0; 28,8,8
| CHANGE /SALES/REVENUE/
| > work/sales-transformed.txt
?

# Step 3: Read and report
PIPE FILE work/sales-transformed.txt
| COUNT
| CONSOLE
?
```

**Use case:** Multi-stage ETL pipeline with intermediate checkpoints.

### Use Case 3: Sorted Employee Listing

```pipe
# Sort employees by name
PIPE CONSOLE
| SORT 0,8,asc
?
# Format output
| SELECT 0,8,0; 8,10,8; 18,10,18; 28,8,28; 36,8,36
?
# Output
| CONSOLE
?
```

**Use case:** Create alphabetically sorted employee roster.

### Use Case 4: Duplicate Removal

```pipe
# Remove duplicate employee records
PIPE CONSOLE
| UNIQ
?
# Sort result
| SORT 0,8,asc
?
# Output
| CONSOLE
?
```

**Use case:** Clean up data, remove duplicates, then sort.

## Success Criteria

1. **CLI demos**: All new demo scripts run successfully
2. **WASM tutorials**: New tutorial steps work in web UI
3. **WASM examples**: Canned examples accessible via dropdown
4. **WASM data loading**: .f80 files can be loaded
5. **Backward compatibility**: All existing `.pipe` files still work
6. **Documentation**: User manual updated with multi-pipeline examples

## References

- [CMS Pipelines Overview (Wikipedia)](https://en.wikipedia.org/wiki/CMS_Pipelines)
- [CMS/TSO Pipelines Runtime Library Distribution](http://vm.marist.edu/~pipeline)
- [edwardaux/Pipes - Swift implementation](https://github.com/edwardaux/Pipes)

## Appendix: Unix Equivalents Reference

| Unix Command | pipelines-rs Stage | Notes |
|---------------|-------------------|-------|
| `sort` | `SORT pos,len,direction` | Sort by field |
| `join` | Future: `JOIN` | Merge records |
| `uniq` | `UNIQ [field]` | Remove duplicates |
| `grep` | `LOCATE /pattern/` or `NLOCATE /pattern/` | Pattern search |
| `sed 's/old/new/'` | `CHANGE /old/new/` | Replace text |
| `awk '{print $1,$3}'` | `SELECT 0,8,0; 28,8,8` | Field extraction |
| `head -n 10` | `TAKE 10` | Take first N |
| `tail -n +10` | `SKIP 10` | Skip first N |
| `wc -l` | `COUNT` | Count records |
