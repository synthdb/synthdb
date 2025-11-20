# SynthDB

> **Production-realistic synthetic data. Zero real information.**

SynthDB generates test data that matches your production database's statistical distributions and relationships—without copying a single real record.

## The Problem

Current tools like Faker.js generate **random** data:
- Names don't match locations ("Rajesh Smith" from "Tokyo")
- Phone numbers don't match countries (New York zip code with Indian area code)
- Ages are uniformly distributed (unrealistic: equal 5-year-olds and 95-year-olds)
- No referential integrity (Orders for users that don't exist)

**Result**: Your tests pass with fake data but fail in production.

## The Solution

SynthDB **learns** your database's patterns:
- **Statistical profiling**: Ages follow a normal distribution (μ=32, σ=8)
- **Smart linking**: Cities match zip codes, area codes match states
- **Referential integrity**: Foreign keys point to valid records
- **Topological sorting**: Inserts data in the correct order (no constraint violations)

## Status

⚠️ **Early Development** - Not ready for production use.

Currently building:
- [x] Project structure
- [ ] Schema parser (MySQL/PostgreSQL)
- [ ] Topological dependency resolver
- [ ] Basic data generation (Faker.js level)
- [ ] Statistical profiling engine
- [ ] Smart relationship linking

**Expected MVP**: January 2025

## Planned Features

### Free (Open Source)
- ✅ MySQL & PostgreSQL support
- ✅ Basic random data generation
- ✅ Topological sorting
- ✅ CLI interface
- ⚠️ Limited to 10,000 rows

### Pro (Paid - Planned)
- 🔒 Statistical profiling (learns distributions)
- 🔒 Smart linking (city ↔ zip code matching)
- 🔒 All databases (Oracle, SQL Server, MongoDB)
- 🔒 Unlimited rows
- 🔒 Docker images
- 🔒 API access

## Quick Start (Coming Soon)
```bash
# Install
npm install -g synthdb

# Connect to your database
synthdb profile mysql://user:pass@localhost/prod_db

# Generate synthetic data
synthdb generate --rows 100000 --output test_db.sql
```

## Roadmap

### Phase 1: MVP (Weeks 1-8)
- Schema introspection
- Topological sorting
- Basic Faker-style generation

### Phase 2: Intelligence (Weeks 9-16)
- Statistical distribution detection
- Smart relationship linking
- Data type inference

### Phase 3: Scale (Weeks 17-24)
- Multi-database support
- Performance optimization
- Enterprise features

## Why This Matters

### GDPR/HIPAA/DPDP Compliance
Using real production data in test environments is:
- ❌ Illegal (€20M fines under GDPR)
- ❌ Dangerous (data breaches from test systems)
- ❌ Slow (anonymization is hard)

SynthDB generates data that's:
- ✅ Legally safe (synthetic = not personal data)
- ✅ Statistically identical (tests behave like production)
- ✅ Fast (no manual anonymization)

## Comparison

| Feature | Faker.js | Tonic.ai | SynthDB |
|---------|----------|----------|---------|
| Price | Free | $50k+/year | Free + $99-$499/mo |
| Statistical accuracy | ❌ | ✅ | ✅ |
| Referential integrity | ❌ | ✅ | ✅ |
| Self-hosted | ✅ | ❌ | ✅ |
| Open source | ✅ | ❌ | ✅ (core) |

## Contributing

🚧 **Not accepting contributions yet** - Core architecture is still being defined.

Follow the repo to be notified when we're ready for contributors!

## Technical Details

**Core Algorithm**:
1. **Schema Analysis**: Parse DB schema, extract tables, columns, foreign keys
2. **Dependency Graph**: Build DAG of table relationships
3. **Topological Sort**: Determine safe insertion order
4. **Statistical Profiling**: Analyze column distributions (mean, std dev, percentiles)
5. **Smart Generation**: Generate data matching learned patterns

**Stack** (Planned):
- Language: Python (using `pandas`, `scipy`, `sqlalchemy`)
- CLI: `click` or `typer`
- Databases: `psycopg2`, `pymysql`, `cx_Oracle`

## License

MIT License - See [LICENSE](LICENSE) for details.

Core engine is open source. Premium features (statistical profiling, enterprise databases) are proprietary.

## Links

- **Website**: Coming soon
- **Docs**: Coming soon
- **Twitter**: Coming soon
- **Discord**: Coming soon

## Star History

⭐ **Star this repo** to follow development and show support!

---

**Built by [@abhinavraj2004](https://github.com/abhinavraj2004)**

*Inspired by the limitations of Faker.js and the high cost of Tonic.ai*
