SynthDB

The Statistical Synthetic Data Engine for Modern Engineering Teams.

SynthDB is an open-source infrastructure tool that generates statistically accurate, referentially intact synthetic data for PostgreSQL and MySQL. It solves the "Cold Start" problem for testing environments by cloning the shape of your production data without touching the PII (Personally Identifiable Information).

The Problem

Developers need realistic data to test features.

Random Data (Faker.js) breaks business logic.

Production Data violates GDPR/DPDP/HIPAA.

The Solution: Statistical Reflection

SynthDB connects to your production schema and performs a 3-step analysis:

Graph Topology Analysis: Builds a Directed Acyclic Graph (DAG) of your Foreign Keys to determine the mathematically correct insertion order.

Statistical Profiling: Analyzes distribution curves (Bell curves, Power laws) of your numerical data.

Synthesis: Streams millions of rows that respect these constraints.

Installation

pip install synthdb


Usage

# 1. Introspect your database and generate a blueprint
synthdb introspect --db-url "postgresql://user:pass@localhost:5432/prod_db"

# 2. Generate synthetic data
synthdb generate --blueprint ./blueprint.json --count 10000


Architecture

SynthDB is built on:

NetworkX for dependency graph resolution.

SQLAlchemy for schema reflection.

Python 3.9+ for core logic.

License

MIT
