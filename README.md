# 🦀 SynthDB

> **Production-grade synthetic data generator for PostgreSQL**  
> **Zero config. Single binary. Referentially intact.**

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Collective](https://img.shields.io/badge/Sponsor-Open%20Collective-1f87ff.svg)](https://opencollective.com/synthdb)

---

## 🎯 Overview

**SynthDB** generates realistic synthetic data for PostgreSQL databases by reading your schema and producing statistically accurate insert statements — while respecting foreign keys and relational dependencies.

It’s designed to solve the **“Staging Data Problem”**:
> *How do I get realistic 100k+ rows into staging without copying sensitive production data?*

---

## 🚀 Key Features

| Feature | Traditional Seeder Scripts | **SynthDB** |
|--------|----------------------------|-------------|
| Setup Time | Hours | **Seconds** |
| Handles FK Relationships | Manual logic | **Automatic DAG sorting** |
| Data Distribution | Randomized | **Sampled from real values** |
| Performance | Slow loops (Node/Python) | **Native Rust speed** |
| Privacy | Potentially risky | **Air-gapped / local only** |

---

## 📦 Installation

Currently installable via Cargo:

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/synthdb.git
cd synthdb

# Install globally
cargo install --path .
🛠 Usage Guide
SynthDB does not create tables — it fills an existing schema.

📌 Example Flow: Clone Prod → Staging With Fake Data
Step 1: Export schema (no data)
bash
Copy code
pg_dump -h prod-db.com -U user -d my_app -s > schema.sql
Step 2: Create a target database
bash
Copy code
createdb my_staging_db
psql -d my_staging_db < schema.sql
Step 3: Generate synthetic data
bash
Copy code
synthdb clone \
  --url postgres://user:pass@localhost:5432/my_staging_db \
  --rows 5000 \
  --output seed.sql
Step 4: Apply generated data
bash
Copy code
psql -d my_staging_db < seed.sql
🎉 Result
A fully-seeded staging environment with realistic users, orders, items, etc. — all referentially intact.

🧠 How It Works
1️⃣ Smart Column Heuristics (“Vibe Engine”)
Column Name	Example Output
email	jim.halpert@example.com
phone	+1 402-555-0198
sku	PROD-4281
status	active / pending / failed
created_at	time-decayed timestamps

2️⃣ Value Sampling
Extracts real database distributions automatically, e.g.:

sql
Copy code
SELECT DISTINCT product_category FROM products;
3️⃣ Topological Sort
Builds a dependency graph to seed tables in valid order.

bash
Copy code
users → orders → order_items
⚠️ Limitations (v0.1)
Capability	Status
PostgreSQL Support	✅ Available
MySQL / SQLite	🚧 Coming in v0.2
Schema must exist	✔ Required
Generates INSERT only	✔ Does not truncate

🤝 Contributing
Contributions welcome!

bash
Copy code
git checkout -b feature/my-feature
# commit changes
git push origin feature/my-feature
Open a PR and we’ll review quickly.

💙 Support & Sponsors
If SynthDB saves you hours of staging setup, consider sponsoring development:

👉 Open Collective: https://opencollective.com/synthdb

📜 License
Distributed under the MIT License — free for commercial and private use.