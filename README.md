# Scrub-DB - Database Anonymization Tool (Free Version)

**Fast, manual database anonymization for development and testing.**

## What is Scrub-DB?

Scrub-DB is a powerful database anonymization engine that helps you safely anonymize SQL dumps. The **free version** provides manual configuration via YAML files, while **Pro** offers automatic PII detection and live database connections.

### Free Version Features

- ✅ **Manual Configuration** - Define anonymization rules via `scrub-db.yaml`
- ✅ **Relationship Preservation** - Same input always produces same output (maintains referential integrity)
- ✅ **3 Anonymization Methods**:
  - Realistic fake data (emails, names, phones)
  - Secure masking (credit cards, SSNs)
  - Consistent hashing (for any sensitive data)
- ✅ **Stdin/Stdout Support** - Pipe SQL dumps directly through the tool
- ✅ **Auto Config Detection** - Automatically finds `scrub-db.yaml` in your working directory
- ✅ **Scan Command** - Preview what PII would be detected (Pro feature teaser)

## Quick Start

```bash
# Install
cargo install scrub-db

# 1. Scan SQL dump to see what PII would be detected (Pro preview)
cat dump.sql | scrub-db scan

# 2. Create a config file with your anonymization rules
cat > scrub-db.yaml <<EOF
preserve_relationships: true
custom_rules:
  email: fake_email
  phone: fake_phone
  credit_card: mask_credit_card
EOF

# 3. Anonymize SQL dump
cat dump.sql | scrub-db > anonymized.sql

# Or pipe directly from pg_dump
pg_dump mydb | scrub-db > safe-dump.sql

# Use custom config file location
cat dump.sql | scrub-db -c my-config.yaml > anonymized.sql
```

## How It Works

### 1. Manual Configuration (Free Version)

Create a `scrub-db.yaml` file with your anonymization rules:

```yaml
preserve_relationships: true
custom_rules:
  email: fake_email
  phone: fake_phone
  ssn: mask_ssn
  credit_card: mask_credit_card
```

**Available Methods:**
- `fake_email` - Generate realistic fake emails
- `fake_name` - Generate realistic fake names
- `fake_phone` - Generate realistic fake phone numbers
- `fake_address` - Generate realistic fake addresses
- `mask_credit_card` - Mask all but last 4 digits
- `mask_ssn` - Completely mask SSNs
- `hash` - SHA-256 hash of the value
- `skip` - Leave unchanged

### 2. Relationship Preservation

When enabled (default), the same input always generates the same output:

```
john.doe@example.com → alice.smith@example.com
john.doe@example.com → alice.smith@example.com  (same!)
```

This preserves foreign key relationships and data integrity.

### 3. Scan Command (Pro Feature Preview)

The free version includes a `scan` command that shows you what PII would be automatically detected in the Pro version:

```bash
$ cat dump.sql | scrub-db scan

🔍 Scrub-DB Scan - PII Detection Preview
=========================================

✨ Scan Results:
   📧 3 lines with potential email addresses
   📱 3 lines with potential phone numbers
   💳 0 lines with potential credit card numbers

🚀 Upgrade to Scrub-DB Pro for automatic detection!
```

This helps you write your manual config rules.

## CLI Reference

```
scrub-db [OPTIONS] [COMMAND]

Commands:
  scan    Scan SQL dump for potential PII (Pro feature preview)

Options:
  -c, --cfg <FILE>  Config file (auto-detects scrub-db.yaml if not specified)
      --stdin       Force stdin mode (auto-detected by default)
  -h, --help        Print help
  -V, --version     Print version
```

**Usage:**

```bash
# Anonymize with config file
cat dump.sql | scrub-db > anonymized.sql

# Scan for PII
cat dump.sql | scrub-db scan

# Use specific config file
cat dump.sql | scrub-db -c custom.yaml > anonymized.sql
```

## Upgrade to Pro

**Want more power? Scrub-DB Pro includes:**

| Feature | Free | Pro |
|---------|------|-----|
| Manual config (YAML) | ✅ | ✅ |
| Stdin/stdout processing | ✅ | ✅ |
| Relationship preservation | ✅ | ✅ |
| **Automatic PII detection** | ❌ | ✅ |
| **Live database connections** | ❌ | ✅ |
| **Database-to-database copy** | ❌ | ✅ |
| **Schema introspection** | ❌ | ✅ |
| **Smart column analysis** | ❌ | ✅ |
| **Cloud DB support (RDS, Cloud SQL)** | ❌ | ✅ |
| **Priority support** | ❌ | ✅ |

**Pricing:**
- 💰 **Pro**: $49/month - For teams of 2-10 developers
- 🏢 **Enterprise**: Custom pricing - Compliance dashboard, SSO, audit logs

**[Visit https://scrub-db.com to upgrade →](#)**

## Example Usage

**1. First, scan to see what PII is present:**

```bash
$ cat test-dump.sql | scrub-db scan

🔍 Scrub-DB Scan - PII Detection Preview
✨ Scan Results:
   📧 3 lines with potential email addresses
   📱 3 lines with potential phone numbers
```

**2. Create config file based on scan:**

```bash
$ cat > scrub-db.yaml <<EOF
preserve_relationships: true
custom_rules:
  email: fake_email
  phone: fake_phone
EOF
```

**3. Anonymize the dump:**

```bash
$ cat test-dump.sql | scrub-db

INSERT INTO users (id, email, phone) VALUES (1, 'adrain@example.com', '555-123-4567');
INSERT INTO users (id, email, phone) VALUES (2, 'kaitlin@example.org', '555-987-6543');
INSERT INTO users (id, email, phone) VALUES (3, 'adrain@example.com', '555-555-5555');
```

**Notice**: `john.doe@example.com` became `adrain@example.com` in **both** rows 1 and 3 - relationship preservation in action!

## Use Cases

- Share production dumps with your team safely
- Create realistic test data from production
- GDPR/privacy compliance
- Debug with real-ish data structures
- Staging environment setup

## Testing

The project includes comprehensive unit tests covering all critical functionality:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_detect_postgres_from_sql
```

**Test Coverage:**
- ✅ 20 unit tests
- ✅ 100% pass rate
- ✅ 0.04s execution time
- ✅ PII detection (column names + data patterns)
- ✅ Anonymization (relationship preservation, masking)
- ✅ Database type detection (SQL syntax + URLs)
- ✅ Configuration defaults

## Development Roadmap

**Free Version (v0.1.0 - Current):**
- [x] Core anonymization engine
- [x] Stdin/stdout support for SQL dumps
- [x] Auto-config file detection (`scrub-db.yaml`)
- [x] Manual configuration via YAML
- [x] Relationship preservation
- [x] 6 anonymization methods (fake, mask, hash)
- [x] `scan` command (Pro feature preview)
- [x] Comprehensive test suite

**Pro Version (In Development):**
- [ ] Automatic PII detection (no config needed)
- [ ] Live database connections (PostgreSQL, MySQL, SQLite)
- [ ] Schema introspection
- [ ] Database-to-database copying
- [ ] Smart column name analysis
- [ ] Advanced pattern matching
- [ ] Cloud database support (AWS RDS, Google Cloud SQL)

**Enterprise Version (Planned):**
- [ ] Compliance dashboard
- [ ] Audit logging
- [ ] SSO / SAML authentication
- [ ] On-premise deployment
- [ ] Priority support & SLAs
- [ ] Custom faker plugins
- [ ] Team collaboration features

## License

MIT OR Apache-2.0
