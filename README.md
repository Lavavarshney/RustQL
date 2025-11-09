# **RustQL** – *Mini SQL Execution Engine in Rust*

A **high-performance, in-memory SQL-like query engine** implemented in **Rust**, featuring **full CRUD semantics**, **symbolic column resolution**, and **zero-downtime persistence** via binary serialization.

Designed for **systems programming education**, **embedded data processing**, and **prototyping lightweight persistence layers**.

---

## Core Technical Capabilities

| Feature | Implementation |
|-------|----------------|
| `CREATE TABLE` | Schema definition with named columns |
| `INSERT INTO` | Row ingestion (i32, &str) |
| `SELECT` | Projection over `*` or named/positional columns |
| `UPDATE` | In-place mutation with conditional filtering |
| `DELETE` | Row eviction via equality predicates |
| **Column Resolution** | Dual-mode: symbolic (`name`) + positional (`colN`) |
| **Persistence** | `bincode`-serialized `database.bin` |
| **Auto-Save** | Post-execution flush on success |
| **REPL CLI** | Interactive command loop with debug introspection |

---

## Architecture Overview

```
src/
├── main.rs        → REPL + lifecycle (load/save)
├── parser.rs      → Lexical analysis + recursive descent parsing
└── executor.rs    → Query execution, table ops, persistence
```

- **Parser**: Hand-rolled **LL(1)** tokenizer + parser (no external crates)
- **Executor**: In-memory `HashMap<String, Table>` with `Vec<Vec<Value>>` storage
- **Serialization**: `serde` + `bincode` for compact, type-safe persistence
- **Data Model**: `Value::Int(i32) | Value::Str(String) | Value::Star | Value::Identifier`

---

## Getting Started

### Prerequisites
```bash
rustc >= 1.70
cargo
```

### Build & Run
```bash
git clone https://github.com/lavavarshney/rustql.git
cd rustql
cargo run --release
```

> First run: `Starting with new database`  
> Subsequent: `Loaded existing database from database.bin`

---

## REPL Interface

```text
Mini SQL Engine - Enter SQL commands (type 'quit' to exit)
Supported: CREATE | INSERT | SELECT | UPDATE | DELETE

> CREATE TABLE metrics (ts, cpu, mem);
OK
> INSERT INTO metrics VALUES (1700000000, 78, 4096);
OK
> SELECT cpu, mem FROM metrics WHERE ts = 1700000000;
["78", "4096"]
OK
> UPDATE metrics SET cpu = 45 WHERE mem > 4000;
Updated 1 rows
OK
> quit
Database saved to database.bin
```

---

## SQL Dialect Specification

### `CREATE TABLE`
```sql
CREATE TABLE t (c1, c2, c3);
```
→ Allocates schema vector; column names stored for symbolic lookup.

### `INSERT INTO`
```sql
INSERT INTO t VALUES (1, 'data', 3.14);
```
→ Appends row; type inference at parse time.

### `SELECT`
```sql
SELECT * FROM t;
SELECT col0, name FROM t;
SELECT cpu FROM metrics;
```
→ Supports `*` expansion and dual-resolution column projection.

### `UPDATE`
```sql
UPDATE t SET col1 = 'new' WHERE id = 1;
UPDATE t SET cpu = 99 WHERE ts = 1700000000;
```
→ Parses `SET` and `WHERE` clauses; applies mutation in-place.

### `DELETE`
```sql
DELETE FROM t WHERE status = 'inactive';
DELETE FROM t WHERE col0 = 42;
```
→ Equality-based retention filter with debug tracing.

---

## Persistence Layer

- **File**: `database.bin` (project root)
- **Format**: `bincode` v1.3 (LEB128 + varint)
- **Strategy**: 
  - Load on startup (`Database::load`)
  - Auto-save post-execution
  - Manual trigger via `save`
- **Integrity**: Atomic write via `std::fs::write`

---

## Column Resolution Engine

```rust
resolve_column(name: &str, schema: &[String]) -> usize
```

1. **Symbolic Match**: `schema.iter().position(|c| c == name)`
2. **Positional Fallback**: `name.starts_with("col") → parse index`
3. **Default**: `0` (fail-soft)

Enables **backward compatibility** with positional syntax.

---

## Special REPL Commands

| Command | Function |
|-------|----------|
| `save` | Force persistence flush |
| `quit` | Graceful shutdown + save |
| `debug` | Dump internal state (schema, rows, types) |

---

## Example: End-to-End Workflow

```sql
CREATE TABLE sensors (id, type, value);
INSERT INTO sensors VALUES (1, 'temp', 23);
INSERT INTO sensors VALUES (2, 'hum', 65);
SELECT type, value FROM sensors;
UPDATE sensors SET value = 24 WHERE id = 1;
DELETE FROM sensors WHERE value < 50;
SELECT * FROM sensors;
save
quit
```

---

## Current Limitations & Extension Points

| Feature | Status | Notes |
|-------|--------|-------|
| `WHERE` logic | `=` only | No `AND`/`OR`/`>/</!=` |
| Schema enforcement | None | No type checking |
| Indexing | Not supported | O(n) scans |
| Transactions | Not supported | No rollback |
| Concurrency | Single-threaded | REPL-only |

**Future Extensions**:
- B+ tree indexing
- Query optimizer (projection pushdown)
- `JOIN` support
- SQL AST validation
- Unit test suite

---

## Testing & Validation

- [`DEMO_SESSION.md`](./DEMO_SESSION.md) – End-to-end sessions
- [`QUICK_REFERENCE.md`](./QUICK_REFERENCE.md) – Syntax cheat sheet
- [`TEST_EXAMPLES.md`](./TEST_EXAMPLES.md) – Regression scenarios

---

## Build Artifacts

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
```

- **Zero runtime dependencies** beyond `std`
- **No heap allocation in hot paths** (except row vectors)

---

## Contributing

We welcome systems-level contributions:

- Add predicate pushdown
- Implement `LIKE`, `IN`, comparison ops
- Add schema validation (int-only columns)
- Write property-based tests (`proptest`)
- Benchmark scan performance

---

## License

```
MIT License
```

---

