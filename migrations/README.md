# 🗄️ Database Migrations

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MySQL](https://img.shields.io/badge/mysql-%2300f.svg?style=for-the-badge&logo=mysql&logoColor=white)](https://www.mysql.com/)
[![SQLx](https://img.shields.io/badge/SQLx-orange?style=for-the-badge)](https://github.com/launchbadge/sqlx)

This directory contains database migrations for the **Blog App Backend** built with Rust and SQLx.

## 📁 Migration Files

| File | Description |
|------|-------------|
| `20241220000001_create_posts_table.up.sql` | ⬆️ Creates the posts table with all necessary columns and indexes |
| `20241220000001_create_posts_table.down.sql` | ⬇️ Drops the posts table for rollback operations |

## ⚡ How Migrations Work

Migrations are **automatically executed** when the application starts using SQLx's embedded migration system:

- The `sqlx::migrate!` macro embeds migration files directly into the binary
- Migrations run in chronological order based on timestamps
- Each migration is tracked in the `_sqlx_migrations` table
- Failed migrations prevent application startup

## 🛠️ Creating New Migrations

### Prerequisites

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features native-tls,mysql
```

### Step-by-Step Guide

1. **Create a new migration:**
   ```bash
   sqlx migrate add <descriptive_migration_name>
   ```

2. **Edit the generated files:**
   - `<timestamp>_<name>.up.sql` - Forward migration
   - `<timestamp>_<name>.down.sql` - Rollback migration

3. **Test your migration:**
   ```bash
   ./migrate.sh run
   ```

4. **Restart the application** - migrations will be applied automatically

## 🎮 Migration Management

### Using the Migration Script

```bash
# Run all pending migrations
./migrate.sh run

# Revert the last applied migration
./migrate.sh revert

# Check current migration status
./migrate.sh info

# Create a new migration
./migrate.sh add "add_user_table"
```

### Using SQLx CLI Directly

```bash
# Run pending migrations
sqlx migrate run --database-url $DATABASE_URL

# Revert last migration
sqlx migrate revert --database-url $DATABASE_URL

# Check migration status
sqlx migrate info --database-url $DATABASE_URL
```

## 📊 Database Schema

### 📝 Posts Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | `VARCHAR(36)` | PRIMARY KEY | Unique identifier (UUID) |
| `title` | `VARCHAR(255)` | NOT NULL | Post title |
| `content` | `TEXT` | NOT NULL | Full post content |
| `excerpt` | `TEXT` | | Short post summary |
| `slug` | `VARCHAR(255)` | UNIQUE, NOT NULL | URL-friendly identifier |
| `author` | `VARCHAR(255)` | NOT NULL | Post author name |
| `published` | `BOOLEAN` | DEFAULT FALSE | Publication status |
| `featured_image` | `VARCHAR(500)` | | Featured image URL |
| `tags` | `JSON` | | Post tags as JSON array |
| `created_at` | `TIMESTAMP` | DEFAULT CURRENT_TIMESTAMP | Creation timestamp |
| `updated_at` | `TIMESTAMP` | DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP | Last update timestamp |

### 🔍 Database Indexes

| Index Name | Column(s) | Purpose |
|------------|-----------|----------|
| `idx_slug` | `slug` | Fast post lookup by URL slug |
| `idx_published` | `published` | Efficient filtering of published posts |
| `idx_created_at` | `created_at` | Chronological sorting and pagination |

## 🚀 Best Practices

- ✅ Always create both `up` and `down` migrations
- ✅ Test migrations on a copy of production data
- ✅ Use descriptive migration names
- ✅ Keep migrations atomic and reversible
- ✅ Never modify existing migration files after deployment
- ❌ Don't delete migration files
- ❌ Avoid complex data transformations in migrations

## 🔧 Troubleshooting

### Common Issues

**Migration fails to run:**
```bash
# Check database connection
sqlx migrate info

# Verify DATABASE_URL in .env file
echo $DATABASE_URL
```

**Application won't start:**
- Check migration syntax in SQL files
- Ensure database is accessible
- Verify all dependencies are installed

**Need to reset migrations:**
```bash
# ⚠️ WARNING: This will drop all data!
DROP TABLE _sqlx_migrations;
```

---

> 💡 **Tip:** Always backup your database before running migrations in production!