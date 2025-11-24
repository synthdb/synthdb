# SynthDB 🦀

> **Production-grade synthetic data generator. Zero config. Single binary.**

[![Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

**SynthDB** is a modern database cloning tool that reads your production schema and generates statistically realistic, referentially intact synthetic data. It solves the "Staging Data Problem" without compromising user privacy.

---

## 🚀 Why SynthDB?

Most synthetic data tools require writing hundreds of lines of "Faker" scripts or paying $50k/year for enterprise software.

SynthDB is different:
1.  **It reads your mind (Schema):** Auto-detects foreign keys, types, and constraints.
2.  **It respects Physics (Graph Theory):** Uses topological sorting to insert data in the correct order (e.g., Users → Orders → Items).
3.  **It creates Vibes (Smart Seeding):** Detects column names (`email`, `phone`, `sku`) and generates matching realistic data.
4.  **It keeps secrets:** Zero user data leaves your server. Everything is generated locally.



[Image of database schema extraction and synthetic data generation process]


---

## ⚡ Quick Start

### 1. Install
```bash
# From source (currently)
git clone [https://github.com/yourusername/synthdb](https://github.com/yourusername/synthdb)
cd synthdb
cargo install --path .