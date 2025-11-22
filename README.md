# Schemly

**Version 2.0.0**

A powerful Laravel code generator written in Rust that creates models, controllers, resources, factories, migrations, and pivot tables from Prisma-like schema files.

🌐 **Website**: [https://schemly.dev/](https://schemly.dev/)

## Features

- 🚀 **Fast & Reliable** - Built with Rust for maximum performance
- 📝 **Prisma-like Syntax** - Define your models with modern, industry-standard syntax
- 🔧 **Complete Laravel Support** - Generates all Laravel components
- 🔗 **Relationship Support** - Full support for all Laravel relationship types
- 🏗️ **Pivot Tables** - Automatic pivot table generation for many-to-many relationships
- 🤖 **AI-Friendly** - Comprehensive documentation designed for AI code generation
- 🏛️ **DDD Support** - Optional Domain-Driven Design folder structure


## Installation

### From Source

```bash
git clone https://github.com/DevPlus31/schemly.git
cd schemly
cargo build --release
```

The binary will be available at `target/release/schemly`.

### Add to PATH (Optional)

```bash
# Copy to a directory in your PATH
cp target/release/schemly /usr/local/bin/
# or
cp target/release/schemly ~/.local/bin/
```

## Quick Start

### 1. Initialize a new schema file

```bash
schemly init
```

This creates a `schema.schemly` file with example models.

### 2. Define your models

Edit `schema.schemly` with Prisma-like syntax:

```prisma
generator laravel {
  provider = "schemly"
  output   = "./app"
}

datasource db {
  provider = "mysql"
  url      = env("DATABASE_URL")
}

model User {
  id        Int      @id @default(autoincrement())
  name      String   @db.VarChar(255)
  email     String   @unique @db.VarChar(255)
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")

  posts     Post[]

  @@map("users")
  @@traits(["HasFactory", "Notifiable"])
  @@fillable(["name", "email"])
}

model Post {
  id        Int      @id @default(autoincrement())
  title     String   @db.VarChar(255)
  content   String   @db.LongText
  userId    Int      @map("user_id")
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")

  user      User     @relation(fields: [userId], references: [id], onDelete: Cascade)

  @@map("posts")
  @@traits(["HasFactory"])
  @@fillable(["title", "content", "user_id"])
}
```

### 3. Generate Laravel files

```bash
# Generate all components
schemly generate

# Preview what would be generated (dry run)
schemly generate --dry-run

# Generate in specific Laravel project
schemly generate --output /path/to/laravel-project

# Generate only specific components
schemly generate --only models,migrations

# Force overwrite existing files
schemly generate --force

# Use Domain-Driven Design structure
schemly generate --ddd
```

## CLI Commands

### `schemly init`

Creates a default `schema.schemly` file with example models.

```bash
# Create schema.schemly in current directory
schemly init

# Create with custom name
schemly init --output my-schema.schemly

# Force overwrite if file exists
schemly init --force
```

### `schemly generate`

Compiles the schema into Laravel code.

```bash
# Generate all components (reads schema.schemly by default)
schemly generate

# Use custom schema file
schemly generate --file my-schema.schemly

# Generate in specific Laravel project
schemly generate --output /path/to/laravel-project

# Preview what would be generated (dry run)
schemly generate --dry-run

# Force overwrite existing files
schemly generate --force

# Generate only specific components
schemly generate --only models,migrations
schemly generate --only controllers,resources,factories

# Use Domain-Driven Design structure
schemly generate --ddd

# Interactive mode (select models and components)
schemly generate --interactive

# Verbose output
schemly generate --verbose
```

**Available components for `--only` flag:**
- `models` - Eloquent models
- `migrations` - Database migrations
- `controllers` - API controllers
- `resources` - API resources
- `factories` - Model factories
- `dtos` - Data Transfer Objects
- `pivot` - Pivot tables

### `schemly watch`

Watches the schema file and auto-generates on save (coming soon).

```bash
schemly watch
schemly watch --file my-schema.schemly
```

### `schemly doctor`

Checks your Laravel project for compatibility.

```bash
# Check current directory
schemly doctor

# Check specific Laravel project
schemly doctor --path /path/to/laravel-project
```

## Examples

Schemly comes with three comprehensive examples to get you started:

### 📱 **Linktree Example** (`examples/linktree.schema`)
A simple social media link aggregator similar to Linktree:
- **Models**: User, Link, Category
- **Features**: Basic relationships, validation rules, soft deletes
- **Perfect for**: Learning Schemly basics

```bash
schemly generate --file examples/linktree.schema
```

### 🛒 **E-commerce Example** (`examples/ecommerce.schema`)
A complete online store system:
- **Models**: User, Category, Product, Order, OrderItem, Review, Address, Image
- **Features**: Complex relationships, decimal pricing, polymorphic images, enums
- **Perfect for**: Production e-commerce applications

```bash
schemly generate --file examples/ecommerce.schema
```

### 📝 **Blog Example** (`examples/blog.schema`)
An advanced content management system:
- **Models**: User, Category, Tag, Post, Comment, Media, Newsletter, Subscriber
- **Features**: DDD structure, many-to-many relationships, polymorphic comments, SEO fields
- **Perfect for**: Content-heavy applications

```bash
schemly generate --file examples/blog.schema
```

## Schema Syntax

Schemly uses a Prisma-like schema syntax that's modern, readable, and industry-standard.

### Basic Structure

```prisma
// Generator configuration
generator laravel {
  provider = "schemly"
  output   = "./app"
}

// Database configuration
datasource db {
  provider = "mysql"
  url      = env("DATABASE_URL")
}

// Enum definition
enum UserRole {
  admin
  editor
  author
}

// Model definition
model User {
  // Fields
  id        Int      @id @default(autoincrement())
  name      String   @db.VarChar(255)
  email     String   @unique
  role      UserRole @default(author)

  // Relationships
  posts     Post[]

  // Model attributes
  @@map("users")
  @@traits(["HasFactory", "Notifiable"])
  @@fillable(["name", "email", "role"])
  @@softDeletes
}
```

### Field Types

- `String` - VARCHAR
- `Int` - INTEGER
- `BigInt` - BIGINT
- `Float` - FLOAT
- `Decimal` - DECIMAL
- `Boolean` - BOOLEAN
- `DateTime` - DATETIME/TIMESTAMP
- `Json` - JSON
- `Bytes` - BLOB

### Field Attributes

- `@id` - Primary key
- `@default(value)` - Default value
- `@unique` - Unique constraint
- `@map("column_name")` - Custom column name
- `@updatedAt` - Auto-update timestamp
- `@db.VarChar(255)` - Database-specific type
- `@validate("rules")` - Laravel validation rules
- `@relation(...)` - Relationship definition

### Model Attributes

- `@@map("table_name")` - Custom table name
- `@@traits([...])` - Laravel traits
- `@@fillable([...])` - Mass assignable fields
- `@@guarded([...])` - Guarded fields
- `@@softDeletes` - Soft delete support
- `@@timestamps` - Created/updated timestamps

### Relationships

```prisma
// One-to-Many
model User {
  posts Post[]
}

model Post {
  userId Int  @map("user_id")
  user   User @relation(fields: [userId], references: [id], onDelete: Cascade)
}

// Many-to-Many (automatic pivot table)
model Post {
  tags Tag[]
}

model Tag {
  posts Post[]
}
```

For complete syntax reference, see `docs/GRAMMAR_COMPARISON.md`.

## Configuration Precedence

Schemly follows a three-level configuration system:

### Level 1: Defaults (Hardcoded in Rust)
- Don't overwrite customized controllers
- Generate all components by default
- Use traditional Laravel folder structure

### Level 2: Schema File Config Block
Project-wide settings defined in your schema file:

```prisma
generator laravel {
  provider = "schemly"
  output   = "./app"
}

datasource db {
  provider = "mysql"
  url      = env("DATABASE_URL")
}
```

### Level 3: CLI Arguments (Runtime Overrides)
Command-line flags override everything:

```bash
# Override output directory
schemly generate --output /custom/path

# Override safety checks
schemly generate --force

# Override component selection
schemly generate --only models,migrations
## Generated Files

Schemly generates the following Laravel files:

### Models (`app/Models/`)

- Eloquent model classes with relationships, casts, and fillable fields
- Proper namespace and imports
- Trait usage (HasFactory, SoftDeletes)

### Controllers (`app/Http/Controllers/`)

- Resource controllers with CRUD operations
- Proper imports and type hints

### Resources (`app/Http/Resources/`)

- API resource classes for JSON responses
- All model fields included

### Factories (`database/factories/`)

- Model factories for testing and seeding
- Intelligent fake data generation based on field names and types

### Migrations (`database/migrations/`)

- Database migration files with proper timestamps
- All field types, constraints, and indexes
- Foreign key relationships

### Pivot Tables (`database/migrations/`)

- Pivot table migrations for many-to-many relationships
- Additional fields and timestamps support

## Safety Features

### File Overwrite Protection

By default, schemly will **NOT** overwrite existing files:

```bash
# Safe - skips existing files with warnings
schemly --config models.yml

# Output:
# Warning: File already exists, skipping: ./app/Models/User.php
# Generated model: Post
#
# Summary:
#   ✓ 1 files generated successfully
#   ⚠ 1 files skipped (already exist)
#   Total: 2 files processed
```

Use `--force` to overwrite existing files:

```bash
# Dangerous - overwrites existing files
schemly --config models.yml --force

# Output:
# ⚠️  Warning: --force flag enabled. Existing files will be overwritten!
# Generated model: User
# Generated model: Post
```

### Validation

Schemly validates your configuration:

- Model names cannot be empty
- Table names cannot be empty
- At least one component type must be enabled
- Enum fields must have enum values
- Relationship configurations are validated

## Error Handling

Schemly provides clear error messages:

```bash
# Invalid configuration
Error: ModelValidation("Model name cannot be empty")

# File permission issues
Error writing ./app/Models/User.php: Permission denied

# Invalid flag combinations
Error: At least one component type must be enabled for generation
```

## Tips & Best Practices

### 1. Start Small

Begin with a simple model and gradually add complexity:

```prisma
model User {
  id    Int    @id @default(autoincrement())
  name  String @db.VarChar(255)
  email String @unique @db.VarChar(255)

  @@map("users")
}
```

### 2. Use Descriptive Field Names

Field names help generate better fake data:

```prisma
model User {
  firstName   String @db.VarChar(100) @map("first_name")   // Generates fake()->firstName()
  emailAddress String @db.VarChar(255) @map("email_address") // Generates fake()->email()
  phoneNumber String @db.VarChar(20) @map("phone_number")  // Generates fake()->phoneNumber()
}
```

### 3. Use Dry Run First

Always preview what will be generated:

```bash
# Preview before generating
schemly generate --dry-run

# Review the output, then generate for real
schemly generate
```

### 4. Test Before Committing

Always test generated code:

```bash
# Generate in a test directory first
schemly generate --output /tmp/test-laravel --dry-run

# Review what would be generated
# Then generate for real
schemly generate --output /tmp/test-laravel

# Review generated files
ls -la /tmp/test-laravel/app/Models/

# Then generate in your real project
schemly generate --output /path/to/real-project
```

### 5. Version Control Integration

Add schemly to your development workflow:

```bash
# Generate fresh files
schemly generate --force

# Review changes
git diff

# Commit if satisfied
git add .
git commit -m "Regenerate Laravel models from schema"
```

### 6. Use Doctor Command

Check your Laravel project before generating:

```bash
# Check if project is ready
schemly doctor

# Then generate
schemly generate
```

## Troubleshooting

### Common Issues

**Permission Denied**

```bash
# Fix file permissions
chmod 755 /path/to/laravel-project
chmod -R 644 /path/to/laravel-project/app/Models/
```

**Invalid Schema Syntax**

```bash
# Use verbose mode to see detailed parsing errors
schemly generate --verbose

# Or check the schema file manually
cat schema.schemly
```

**Missing Directories**

```bash
# Use doctor command to check Laravel project structure
schemly doctor

# Or ensure Laravel directory structure exists
mkdir -p app/Models database/migrations app/Http/Controllers
```

### Debug Mode

For detailed error information, use Rust's debug output:

```bash
RUST_BACKTRACE=1 schemly generate
```

### Development Setup

```bash
git clone https://github.com/DevPlus31/schemly.git
cd schemly
cargo build
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_model_generation
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Why We Dropped YAML Support

In version 2.0.0, we completely migrated from YAML to Prisma-like schema syntax. Here's why:

### Verbosity Comparison

**YAML (Old - 28 lines):**
```yaml
models:
  - name: User
    table: users
    timestamps: true
    softDeletes: true
    fields:
      - name: name
        type: string
        length: 255
        nullable: false
      - name: email
        type: string
        length: 255
        nullable: false
        unique: true
      - name: password
        type: string
        length: 255
        nullable: false
    relationships:
      - type: hasMany
        model: Post
        foreignKey: user_id
    traits:
      - HasFactory
      - Notifiable
```

**Prisma-like (New - 15 lines - 46% less code!):**
```prisma
model User {
  id        Int      @id @default(autoincrement())
  name      String   @db.VarChar(255)
  email     String   @unique @db.VarChar(255)
  password  String   @db.VarChar(255)
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")

  posts     Post[]

  @@map("users")
  @@traits(["HasFactory", "Notifiable"])
  @@softDeletes
}
```

### Benefits of Prisma-like Syntax

✅ **~60% less verbose** - More concise, easier to read
✅ **Industry standard** - Familiar to developers using Prisma
✅ **Better tooling** - Syntax highlighting, IDE support
✅ **Type safety** - Clear field types and relationships
✅ **Modern syntax** - Attributes instead of nested YAML

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete version history.

### Version 2.0.0 (Latest)

**🎉 Major Release - Prisma-like Schema Syntax**

**Breaking Changes:**
- ⚠️ **Complete migration from YAML to Prisma-like schema format**
- ⚠️ **New CLI structure with subcommands** (`init`, `generate`, `watch`, `doctor`)
- ⚠️ **No backward compatibility with YAML files**
- ⚠️ **Dropped YAML support due to verbosity** (~60% reduction in configuration size)

**New Features:**
- ✅ **Prisma-like Syntax**: Modern, industry-standard schema definition language
- ✅ **New CLI Commands**: `init`, `generate`, `watch`, `doctor`
- ✅ **Dry Run Mode**: Preview what would be generated with `--dry-run`
- ✅ **Doctor Command**: Check Laravel project compatibility
- ✅ **Improved Component Selection**: `--only models,migrations` syntax
- ✅ **Enhanced Parser**: Pest-based grammar with full Prisma compatibility
- ✅ **~60% Reduction**: Less verbose configuration compared to YAML

**Examples Updated:**
- ✅ `examples/linktree.schema` - 3 models with basic relationships
- ✅ `examples/blog.schema` - 8 models with enums and polymorphic relationships
- ✅ `examples/ecommerce.schema` - 8 models with complex relationships

**Documentation:**
- 📚 **Grammar Comparison**: Complete Prisma vs Schemly feature comparison
- 📚 **Migration Guide**: How to convert from YAML to Prisma-like syntax
- 📚 **Updated README**: New CLI commands and schema syntax examples

## Support

- 📖 [Documentation](https://github.com/DevPlus31/schemly/wiki)
- 🐛 [Issue Tracker](https://github.com/DevPlus31/schemly/issues)
- 💬 [Discussions](https://github.com/DevPlus31/schemly/discussions)

---

**Made with ❤️ and ☕ by developers, for developers.**
