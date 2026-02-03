# AGENTS.md

This file provides quick reference information for AI agents working on the pipelines-rs project.

## Project Quick Reference

**Name:** pipelines-rs
**Type:** Rust library + WASM web UI
**Purpose:** Historical mainframe-style batch processing with 80-byte fixed-width records
**Architecture:** Record-oriented, iterator-based data flow
**Demo URL:** https://sw-comp-history.github.io/pipelines-rs/
**Repository:** https://github.com/sw-comp-history/pipelines-rs

## Key Concepts

**Data Model:**
- **Record:** 80-byte fixed-width ASCII records (punch card format)
- **Pipeline:** Iterator-based data processing chain
- **Stage:** Transform/filter/inspect operations
- **DSL:** Text-based pipeline language

**Design Philosophy:**
- ASCII-only (simulating EBCDIC conversion from mainframes)
- Columnar data in fixed positions (not CSV, not JSON)
- Unix/Linux-adapted mainframe patterns
- Pull-based (iterator) data flow - lazy evaluation

## File Structure

```
pipelines-rs/
├── Cargo.toml              # Workspace root (library + wasm-ui)
├── CLAUDE.md              # Claude Code specific guidance
├── AGENTS.md              # This file - quick agent reference
├── src/                   # Main library
│   ├── lib.rs            # Library exports
│   ├── record.rs         # Record type (80-byte fixed-width)
│   ├── pipeline.rs       # Pipeline struct with fluent API
│   ├── stage.rs          # Stage trait + implementations
│   ├── dsl.rs           # DSL parser (PIPE, FILTER, SELECT, etc.)
│   ├── error.rs          # Error types (thiserror)
│   └── main.rs         # Main library entry point
├── wasm-ui/              # WASM web application
│   ├── src/
│   │   ├── app.rs      # Yew main component
│   │   ├── dsl.rs      # DSL parser for web UI
│   │   └── components.rs
│   ├── index.html       # WASM entry point
│   └── dist/           # Built WASM output
├── src/bin/             # CLI binaries
│   └── pipe-run.rs     # CLI tool for running .pipe files
├── specs/               # Sample pipeline files (.pipe)
├── demos/               # Demo shell scripts
├── docs/                # Documentation
│   ├── prd.md         # Product Requirements
│   ├── plan.md        # Development Plan
│   ├── status.md      # Project Status
│   ├── architecture.md # System Design
│   ├── design.md      # Technical Decisions
│   ├── process.md     # Development Workflow
│   ├── user-manual.md # Usage Guide
│   └── multi-stage-pipes-design.md # Multi-stage design
├── pages/               # GitHub Pages (built WASM UI)
├── scripts/             # Build and serve scripts
│   ├── build.sh       # Build library + WASM UI
│   └── serve.sh       # Serve web UI locally (port 9952)
└── tests/              # Integration tests
```

## Critical Commands

**Build Commands (ALWAYS use build.sh):**
```bash
./scripts/build.sh        # Build library + WASM UI (use this!)
cargo build              # Build library only
cargo test               # Run all tests
cargo test test_name      # Run single test
```

**Quality Gates (MANDATORY before commit):**
```bash
cargo test                                    # All tests must pass
cargo clippy --all-targets --all-features -- -D warnings  # Zero warnings
cargo fmt --all                                # Code formatted
markdown-checker -f "**/*.md"                # Markdown validated
```

**Development:**
```bash
# Serve web UI locally
./scripts/serve.sh        # Start server on port 9952
# Browser: http://localhost:9952

# Run CLI demo
cargo run --bin pipe-run --release -- specs/example.pipe specs/input-fixed-80.data

# Run all demos
./demos/demo-all.sh
```

## Architecture Patterns

### Pipeline Execution

**Current (Single Pipeline):**
```
Input Records → Stage 1 → Stage 2 → ... → Output Records
```

**Multi-Stage (Future):**
```
Pipeline 1 → Pipeline 2 → Pipeline 3
  (output) → (input) → (output)
```

### DSL Syntax

**Single Pipeline:**
```pipe
PIPE CONSOLE
| FILTER 18,10 = "SALES"
| SELECT 0,8,0; 28,8,8
| CONSOLE
?
```

**Multi-Pipeline (Chained):**
```pipe
PIPE CONSOLE
| LOCATE /SALES/
?
| SELECT 0,8,0; 28,8,8
?
| CONSOLE
?
```

### Stage Pattern

**Implementing a new stage:**

1. **Add to Command enum** (src/dsl.rs):
```rust
enum Command {
    MyStage { param1: String, param2: usize },
    // ... other variants
}
```

2. **Implement parser** (src/dsl.rs):
```rust
fn parse_command(line: &str) -> Result<Command, String> {
    let upper = line.to_uppercase();
    if upper.starts_with("MYSTAGE ") {
        let rest = line[8..].trim();
        // Parse rest...
    }
    // ... other commands
}
```

3. **Implement executor** (src/dsl.rs):
```rust
fn apply_command(records: Vec<Record>, cmd: &Command) -> Result<Vec<Record>, String> {
    match cmd {
        Command::MyStage { param1, param2 } => {
            // Transform records...
        }
        // ... other commands
    }
}
```

4. **Add to user manual** (docs/user-manual.md)
5. **Add to tutorial** (wasm-ui/src/app.rs TUTORIALS constant)
6. **Write tests**

## Code Quality Rules

**Strict Requirements:**
- **Zero clippy warnings** - Never use `#[allow(...)]` to suppress
- **Files under 500 lines** - Prefer 200-300
- **Functions under 50 lines** - Prefer 10-30
- **Max 3 TODOs per file** - Never commit FIXMEs
- **Inline format args:** `format!("{name}")` not `format!("{}", name)`
- **Doc comments:** `//!` for modules, `///` for items

**TDD Workflow:**
1. Write failing test
2. Implement minimal code to pass
3. Refactor if needed
4. Run quality gates (test, clippy, fmt, markdown-checker)
5. Commit

## Common Tasks

**Adding a new stage:**
1. Define parser variant in `src/dsl.rs`
2. Implement `parse_*` function
3. Implement `apply_*` function
4. Add tests in `src/dsl.rs`
5. Update docs/user-manual.md
6. Update wasm-ui/src/app.rs TutorialStep constants
7. Run quality gates

**Adding a demo:**
1. Create `specs/new-example.pipe`
2. Create `demos/demo-new-example.sh`
3. Add to `demos/demo-all.sh`
4. Run `./demos/demo-all.sh` to verify

**Adding a tutorial step:**
1. Add to `wasm-ui/src/app.rs` TUTORIALS constant
2. Include `example_pipeline` with inline comments
3. Include `description` explaining the concept
4. Run `./scripts/build.sh` to build WASM
5. Test in web UI

**Running quality gates:**
```bash
# All together
cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all && markdown-checker -f "**/*.md"

# Or individually
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
markdown-checker -f "**/*.md"
```

## Decision Log

**Why Iterator-Based Pull Data Flow?**
- Lazy evaluation - only process what's needed
- Composable - can chain stages easily
- Memory efficient - streaming for large datasets
- Unix-like - similar to pipe operators

**Why 80-byte Fixed-Width Records?**
- Historical mainframe punch card format
- Simple - no delimiters to parse
- Fixed positions - fast field access
- Mainframe compatibility - easy migration

**Why DSL Instead of Rust API?**
- Human readable - easier to learn
- Composable - can save/load pipeline specs
- Portable - same .pipe file works in CLI and web UI
- Historical - follows CMS Pipelines syntax

**Why WASM UI?**
- Same code runs everywhere (browser, desktop, mobile)
- No server required - static hosting
- Fast - compiled to native speed
- Safe - Rust's memory safety in browser

## Common Pitfalls

**Don't:**
- ❌ Run `trunk build` directly - use `./scripts/build.sh`
- ❌ Use `#[allow(...)]` - fix the actual issue
- ❌ Commit FIXMEs - use TODOs sparingly
- ❌ Write files > 500 lines - split them up
- ❌ Skip tests - TDD is required
- ❌ Use `format!("{}", var)` - use `format!("{var}")`

**Do:**
- ✅ Always run quality gates before committing
- ✅ Keep functions under 50 lines
- ✅ Use iterators for data transformations
- ✅ Document public APIs
- ✅ Test edge cases
- ✅ Follow Rust idioms

## Current Status

**Phase:** Multi-Stage Pipeline Specifications (Active)

**Milestones:**
- ✅ M1: Core Pipeline (Complete)
- ✅ M2: CLI Interface (Complete)
- 🔄 M3: Multi-Stage Pipeline Specifications (In Progress)
- ⏳ M4: Visual Pipeline Debugger (Post-MVP)
- ⏳ M5: Advanced Features (Future)

**Next Tasks:**
- Implement multi-pipeline parser (?-separated pipelines)
- Add FILE source/sink stages
- Add Unix-style stages (SORT, SPLIT, UNIQ)
- WASM UI enhancements (tutorial submenus, .f80 loading)
- Visual debugger (tabbed view, breakpoints, step controls)

## Getting Started for New Agents

1. **Read CLAUDE.md** for detailed guidance
2. **Read docs/plan.md** for current roadmap
3. **Read docs/status.md** for project status
4. **Read docs/user-manual.md** for usage examples
5. **Read docs/multi-stage-pipes-design.md** for current feature focus
6. **Run `./scripts/build.sh`** to verify build works
7. **Run `./scripts/serve.sh`** to see web UI

## References

- **CLAUDE.md** - Detailed Claude Code guidance
- **docs/plan.md** - Development roadmap
- **docs/status.md** - Current progress
- **docs/architecture.md** - System design
- **docs/design.md** - Technical decisions
- **docs/prd.md** - Product requirements
- **docs/process.md** - Development workflow
- **docs/user-manual.md** - Usage guide
- **docs/multi-stage-pipes-design.md** - Multi-stage feature design
