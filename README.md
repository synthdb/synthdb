# SynthDB 🦀

> The Universal Database Seeder.  
> Production-grade synthetic data. Zero config. Context-aware.

[![Crates.io](https://img.shields.io/crates/v/synthdb.svg)](https://crates.io/crates/synthdb)
[![Built with Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Collective](https://img.shields.io/badge/Sponsor-Open%20Collective-1f87ff.svg)](https://opencollective.com/synthdb)

SynthDB reads your PostgreSQL schema and synthesizes realistic, relationally consistent data — with no config, no templates, and minimal fuss. It infers semantics from column names and types, preserves foreign keys, and emits performant INSERTs (or, in future, writes directly).

Why SynthDB?
- Zero-config: sensible heuristics that just work.
- Context-aware: column semantics (emails, names, entities) produce believable values.
- Referential integrity: inserts in topological order; no broken FK references.
- Privacy-first: run locally; no telemetry or data exfiltration by default.

Quick start

1) Install
```bash
# From crates.io
cargo install synthdb
```

2) Prepare a target DB (schema only)
```bash
# Export schema from prod (no data)
pg_dump -h prod-db.example -U user -d my_app -s > schema.sql

# Create a staging DB and load schema
createdb my_staging_db
psql -d my_staging_db < schema.sql
```

3) Generate synthetic data
```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_staging_db" \
  --rows 1000 \
  --output seed.sql
```

4) Apply the data
```bash
psql -d my_staging_db < seed.sql
```

Key features

- Universal Heuristic Engine
  - Semantic awareness: uses column names + types to infer generators (first_name → realistic email/local-part).
  - Dynamic entity recognition: creates plausible entity names (e.g., "Kirlin Airport").
  - Precision-aware: respects NUMERIC/DECIMAL scales, VARCHAR limits and other constraints.

- Referential integrity
  - Detects FKs and topologically sorts tables for safe insertion order.
  - Tracks generated primary keys to guarantee referential correctness.

- Rich type support
  - JSONB, ARRAY, UUID, INET, MACADDR, CIDR and more.
  - Media/file hints (e.g., .mp4 for video, .pdf for docs).
  - Network data (MAC, IPv4/IPv6) and code-like patterns (SKU, flight_code).

- Sampling & distributions
  - Optionally sample DISTINCT values from your schema to preserve categorical distributions.
  - Hybrid mode: mix sampled real-world values with synthetic generators.

How it works (high level)
1. Introspect: reads schema metadata (pg_catalog / information_schema).
2. Analyze: infers column semantics and constraints.
3. Build DAG: maps FK relationships and topologically sorts tables.
4. Profile (optional): samples distinct values and builds distributions.
5. Generate: produces rows according to heuristics and sampled distributions.
6. Emit: writes INSERT statements in a FK-safe order.

Common usage / CLI

Basic clone
```bash
synthdb clone --url "postgres://user:pass@localhost:5432/my_db" --rows 5000 --output seed.sql
```

Useful flags
- --url (required): Postgres connection string to introspect
- --rows: number of rows per primary table (defaults sensible per table)
- --output: path to write SQL INSERTs (stdout if omitted)
- --sample-percent: percent of distinct values to sample from existing columns (0–100)
- --concurrency: worker threads for generation/sampling
- --schema: target schema (defaults to public)
- --dry-run: print plan & DAG without generating data
- --help: show all options

Example: faster generation with sampling
```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_db" \
  --rows 20000 \
  --sample-percent 10 \
  --concurrency 8 \
  --output /tmp/seed.sql
```

Column heuristics (examples)
- first_name, last_name → human names
- email, contact_email → realistic email addresses
- phone, mobile → locale-aware formatted phone numbers
- sku, product_code → uppercase alphanumeric patterns with dashes
- created_at, updated_at → time-decayed timestamps
- wallet_balance NUMERIC(10,2) → financial values respecting scale
- airport_facility_name, device_name → entity generators with contextual suffixes

Advanced notes
- Large schemas: sampling many distinct values is DB-intensive; use --sample-percent or increase DB resources.
- Cyclic references: schemas with strong cycles may need manual handling (see --dry-run to inspect plan).
- Performance: generation is CPU-bound; increase --concurrency for multi-core machines.

Roadmap
- v0.2: MySQL & SQLite support
- Native write-mode: write directly into DB (skip SQL file)
- GUI / VSCode extension for schema preview & interactive generation
- Plugin system for custom heuristics and community templates

Contributing
We welcome contributions — especially Rustaceans! Suggested steps:
1. Fork the repo: https://github.com/synthdb/synthdb
2. Create a feature branch: git checkout -b feature/my-feature
3. Implement, test, and document changes
4. Open a pull request

Helpful additions I can generate for you:
- CONTRIBUTING.md
- PR_TEMPLATE.md
- CODE_OF_CONDUCT.md

Support & Sponsorship
If SynthDB saves you time, consider sponsoring development:
- Open Collective: https://opencollective.com/synthdb

License
Distributed under the MIT License. See LICENSE in this repository.

Contact
- Repo & issues: https://github.com/synthdb/synthdb
- Sponsorship: https://opencollective.com/synthdb

If you'd like, I can also:
- add a CONTRIBUTING.md and PR template and open a branch with those files,
- add a brief example of a YAML config schema and a sample config file,
or
- produce a short "Troubleshooting" section with common errors and fixes.
Tell me which and I will add those files next.