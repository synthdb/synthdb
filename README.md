<div align="center">

# 🦀 SynthDB

### **The Universal Database Seeder**
#### Production-grade synthetic data. Zero config. Context-aware.

[![Crates.io](https://img.shields.io/crates/v/synthdb.svg)](https://crates.io/crates/synthdb)
[![Built with Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/synthdb)

[Features](#-features) • [Quick Start](#-quick-start) • [Examples](#-examples) • [Contributing](#-contributing)

---

</div>

## 📖 Overview

**SynthDB** is a next-generation database seeding engine that reads your existing PostgreSQL schema and generates **statistically realistic, relational data** automatically.

Unlike traditional tools that generate random gibberish, SynthDB employs a **Deep Semantic Engine** to understand your data model's context and relationships, producing data that looks and feels real.

```sql
-- Instead of this garbage:
INSERT INTO users VALUES ('XJ9K2', 'asdf@qwerty', '99999', 'ZZZ');

-- SynthDB generates this:
INSERT INTO users VALUES ('John Doe', 'john.doe@techcorp.com', '+1-555-0142', 'San Francisco, CA');
```

---

## ✨ Features

### 🧠 **Deep Semantic Intelligence**

SynthDB understands the *meaning* of your columns, not just their types.

#### 🎯 Context-Aware Identity
If a table has `first_name`, `last_name`, and `email`, SynthDB ensures they match perfectly:
- **Name:** "Sarah Martinez"
- **Email:** "sarah.martinez@company.com"
- **Username:** "smartinez"

#### 🏷️ Smart Categorization
Automatically detects and generates valid data across multiple domains:

<table>
<tr>
<td width="50%">

**💰 Finance**
- Credit Cards (valid Luhn)
- IBANs & Swift Codes
- Cryptocurrency Addresses
- Currency Codes & Amounts

**🌍 Geography**
- Coherent Addresses
- Cities ↔ States ↔ Zip Codes
- Latitude/Longitude Pairs
- Time Zones

**🔬 Science**
- Chemical Formulas
- DNA Sequences
- Medical/ICD Codes
- Laboratory Values

</td>
<td width="50%">

**💻 Technology**
- IPv4 & IPv6 Addresses
- MAC Addresses
- User Agents
- File Paths & URLs

**🏢 Business**
- Company Names
- Job Titles
- Department Names
- Stock Tickers

**📱 Personal**
- Phone Numbers
- Social Security Numbers
- Passport Numbers
- Driver's License IDs

</td>
</tr>
</table>

---

### 🔗 **Referential Integrity**

#### 📊 Topological Sort
Automatically analyzes foreign key dependencies and inserts data in the correct order:
```
Users → Orders → OrderItems → Shipments
```

#### ✅ Zero Broken Links
Generated foreign keys **always** reference valid, existing parent rows. No orphaned records, ever.

```sql
-- Parent record created first
INSERT INTO customers (id, name) VALUES (1, 'Acme Corp');

-- Child record references existing parent
INSERT INTO orders (id, customer_id, total) VALUES (101, 1, 1299.99);
```

---

### 🛡️ **Production Ready**

| Feature | Description |
|---------|-------------|
| **Strict Precision** | Respects `NUMERIC(10,2)`, `VARCHAR(15)`, and all constraint types |
| **Smart Nulls** | Intelligently applies NULL values to optional fields while keeping critical data populated |
| **Unique Constraints** | Guarantees uniqueness for columns with UNIQUE or PRIMARY KEY constraints |
| **Check Constraints** | Honors CHECK constraints and enum types |
| **Zero Configuration** | No YAML files, no mapping rules. Just point it at your database |
| **Performance** | Written in Rust 🦀 for blazing-fast data generation |

---

## ⚡ Quick Start

### 📥 Installation

```bash
# Via Cargo
cargo install synthdb
```

### 🎯 Basic Usage

**Step 1:** Create a target database with your schema (tables must exist)

**Step 2:** Run SynthDB

```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_staging_db" \
  --rows 1000 \
  --output seed.sql
```

**Step 3:** Apply the generated data

```bash
psql -d my_staging_db -f seed.sql
```

### 🔧 Advanced Options

```bash
# Generate data directly to database (no SQL file)
synthdb clone --url "postgres://..." --rows 5000 --execute

# Specify custom row counts per table
synthdb clone --url "postgres://..." --config counts.json

# Exclude specific tables
synthdb clone --url "postgres://..." --exclude "logs,temp_*"

# Set data locale
synthdb clone --url "postgres://..." --locale "en_GB"
```

---

## 💡 Examples

### 🎨 How SynthDB Handles Data

<table>
<thead>
<tr>
<th>Column Name</th>
<th>Generated Value</th>
<th>Logic</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>merchant_name</code></td>
<td><code>'Acme Corporation'</code></td>
<td>🏢 Detected Company entity</td>
</tr>
<tr>
<td><code>support_email</code></td>
<td><code>'support@acmecorp.com'</code></td>
<td>📧 Matched to Company Name</td>
</tr>
<tr>
<td><code>mac_address</code></td>
<td><code>'00:1A:2B:3C:4D:5E'</code></td>
<td>🔧 Valid hexadecimal format</td>
</tr>
<tr>
<td><code>ipv6_address</code></td>
<td><code>'2001:0db8:85a3::8a2e:0370'</code></td>
<td>🌐 Valid IPv6 format</td>
</tr>
<tr>
<td><code>contract_value</code></td>
<td><code>45021.50</code></td>
<td>💯 Respected <code>NUMERIC(10,2)</code></td>
</tr>
<tr>
<td><code>tracking_code</code></td>
<td><code>'TRK-9281-A02'</code></td>
<td>🎯 Semantic ID generation</td>
</tr>
<tr>
<td><code>audit_log_path</code></td>
<td><code>'/var/logs/audit/2024-11.log'</code></td>
<td>📁 Context-aware file path</td>
</tr>
<tr>
<td><code>birth_date</code></td>
<td><code>'1985-06-15'</code></td>
<td>🎂 Realistic age distribution</td>
</tr>
<tr>
<td><code>website_url</code></td>
<td><code>'https://acmecorp.com'</code></td>
<td>🔗 Matched to company domain</td>
</tr>
</tbody>
</table>

### 🗂️ Real-World Schema Example

```sql
-- Your existing schema
CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    website VARCHAR(255),
    industry VARCHAR(50)
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    company_id INTEGER REFERENCES companies(id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(20),
    job_title VARCHAR(100),
    salary NUMERIC(10,2),
    hire_date DATE NOT NULL
);
```

**SynthDB generates:**

```sql
-- Coherent company data
INSERT INTO companies VALUES 
(1, 'TechVision Solutions', 'https://techvision.io', 'Software'),
(2, 'Global Logistics Inc', 'https://globallogistics.com', 'Transportation');

-- Employees with matching company context
INSERT INTO employees VALUES 
(1, 1, 'Alice', 'Chen', 'alice.chen@techvision.io', '+1-555-0123', 'Senior Software Engineer', 125000.00, '2022-03-15'),
(2, 1, 'Bob', 'Kumar', 'bob.kumar@techvision.io', '+1-555-0124', 'Product Manager', 135000.00, '2021-08-22'),
(3, 2, 'Carol', 'Rodriguez', 'carol.rodriguez@globallogistics.com', '+1-555-0198', 'Operations Director', 145000.00, '2020-01-10');
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     SynthDB Engine                      │
├─────────────────────────────────────────────────────────┤
│  1. Schema Introspection                                │
│     └─ Read tables, columns, constraints, relationships │
│                                                          │
│  2. Dependency Analysis                                 │
│     └─ Build dependency graph via topological sort      │
│                                                          │
│  3. Semantic Classification                             │
│     └─ Detect column meaning from names & types         │
│                                                          │
│  4. Context-Aware Generation                            │
│     └─ Generate coherent, relational data               │
│                                                          │
│  5. Constraint Validation                               │
│     └─ Ensure all DB constraints are satisfied          │
│                                                          │
│  6. Output                                              │
│     └─ SQL file or direct database insertion            │
└─────────────────────────────────────────────────────────┘
```

---

## 🗺️ Roadmap

- [x] PostgreSQL support
- [x] Semantic column detection
- [x] Foreign key resolution
- [ ] MySQL/MariaDB support
- [ ] SQLite support
- [ ] Custom data providers
- [ ] GraphQL schema support
- [ ] Performance benchmarking suite
- [ ] Web UI for configuration
- [ ] Machine learning-based pattern detection

---

## 🤝 Contributing

We love Rustaceans! 🦀 Contributions are welcome and appreciated.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```
4. **Commit your changes**
   ```bash
   git commit -m 'Add amazing feature'
   ```
5. **Push to your fork**
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request**

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/synthdb.git
cd synthdb

# Build the project
cargo build

# Run tests
cargo test

# Run with example
cargo run -- clone --url "postgres://localhost/testdb" --rows 100
```

### Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

---


---

## 🙏 Acknowledgments

Built with ❤️ using:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tokio](https://tokio.rs/) - Async runtime
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [Fake](https://github.com/cksac/fake-rs) - Data generation library

---

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.

---

## 💬 Community & Support

- **Issues:** [GitHub Issues](https://github.com/synthdb/synthdb/issues)
- **Discussions:** [GitHub Discussions](https://github.com/synthdb/synthdb/discussions)

---

<div align="center">

**If SynthDB helps your project, consider giving it a ⭐ on GitHub!**

Made with 🦀 by the SynthDB team

</div>