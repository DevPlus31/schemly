# Schemly - Laravel Code Generator

A powerful Laravel code generator written in Rust that creates models, controllers, resources, factories, migrations, and pivot tables from YAML configuration files.

## Features

- 🚀 **Fast & Reliable** - Built with Rust for maximum performance
- 📝 **YAML Configuration** - Define your models in simple, readable YAML
- 🔧 **Complete Laravel Support** - Generates all Laravel components
- 🛡️ **Safe by Default** - Won't overwrite existing files unless forced
- 🎯 **Selective Generation** - Choose exactly which components to generate
- 📊 **Detailed Statistics** - Clear summary of generated, skipped, and failed files
- 🔗 **Relationship Support** - Full support for all Laravel relationship types
- 🏗️ **Pivot Tables** - Automatic pivot table generation for many-to-many relationships

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

1. **Create a YAML configuration file** (e.g., `models.yml`):

```yaml
models:
  - name: User
    table: users
    timestamps: true
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
    relationships:
      - type: hasMany
        model: Post
        foreignKey: user_id

  - name: Post
    table: posts
    timestamps: true
    fields:
      - name: user_id
        type: bigInteger
        nullable: false
      - name: title
        type: string
        length: 255
        nullable: false
      - name: content
        type: text
        nullable: false
    relationships:
      - type: belongsTo
        model: User
        foreignKey: user_id
```

2. **Generate Laravel files**:

```bash
# Generate all components in current Laravel project
schemly --config models.yml

# Generate in specific Laravel project
schemly --config models.yml --output /path/to/laravel-project

# Generate only models and migrations
schemly --config models.yml --only-models --only-migrations
```

## Usage

### Basic Commands

```bash
# Generate all components (default behavior)
schemly --config models.yml

# Generate in specific Laravel project directory
schemly --config models.yml --output /path/to/laravel-project

# Force overwrite existing files
schemly --config models.yml --force

# Show help
schemly --help
```

### Selective Generation

**Generate only specific components:**

```bash
# Only models
schemly --config models.yml --only-models

# Only migrations
schemly --config models.yml --only-migrations

# Models and migrations only
schemly --config models.yml --only-models --only-migrations

# Only controllers and resources
schemly --config models.yml --only-controllers --only-resources
```

**Skip specific components:**

```bash
# Skip controllers
schemly --config models.yml --no-controllers

# Skip factories and resources
schemly --config models.yml --no-factories --no-resources

# Generate everything except pivot tables
schemly --config models.yml --no-pivot-tables
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-c, --config <FILE>` | Path to YAML configuration file | Required |
| `-o, --output <DIR>` | Laravel project root directory | `.` (current directory) |
| `--only-models` | Generate only models | |
| `--only-controllers` | Generate only controllers | |
| `--only-resources` | Generate only resources | |
| `--only-factories` | Generate only factories | |
| `--only-migrations` | Generate only migrations | |
| `--only-pivot-tables` | Generate only pivot tables | |
| `--no-models` | Skip model generation | |
| `--no-controllers` | Skip controller generation | |
| `--no-resources` | Skip resource generation | |
| `--no-factories` | Skip factory generation | |
| `--no-migrations` | Skip migration generation | |
| `--no-pivot-tables` | Skip pivot table generation | |
| `--force` | Force overwrite existing files | |
| `-h, --help` | Show help information | |

## YAML Configuration

### Basic Model Structure

```yaml
models:
  - name: ModelName          # Required: PHP class name
    table: table_name        # Required: Database table name
    timestamps: true         # Optional: Add created_at/updated_at (default: false)
    softDeletes: false       # Optional: Add soft delete support (default: false)
    fields:                  # Required: List of model fields
      - name: field_name
        type: string
        # ... field options
    relationships:           # Optional: Model relationships
      - type: hasMany
        model: RelatedModel
        # ... relationship options
    pivotTables:            # Optional: Pivot tables for many-to-many
      - name: pivot_table_name
        # ... pivot table options
```

### Field Types

Schemly supports all Laravel field types:

```yaml
fields:
  # String types
  - name: title
    type: string
    length: 255              # Optional: field length
    nullable: false          # Optional: allow null (default: false)
    unique: true             # Optional: unique constraint (default: false)
    index: true              # Optional: database index (default: false)
    default: "default_value" # Optional: default value

  # Text types
  - name: description
    type: text               # text, longText, mediumText

  # Numeric types
  - name: age
    type: integer            # integer, bigInteger, tinyInteger, smallInteger, mediumInteger
    unsigned: true           # Optional: unsigned constraint

  - name: price
    type: decimal
    decimalPrecision:        # Optional: decimal precision
      precision: 8
      scale: 2

  # Other types
  - name: is_active
    type: boolean

  - name: birth_date
    type: date               # date, dateTime, timestamp

  - name: metadata
    type: json

  - name: uuid
    type: uuid

  - name: status
    type: enum
    enumValues:              # Required for enum fields
      - value: "active"
        label: "Active"      # Optional: human-readable label
      - value: "inactive"
        label: "Inactive"

  - name: ip_address
    type: inet               # IP address field

  - name: file_data
    type: binary             # Binary data
```

### Relationships

Schemly supports all Laravel relationship types:

```yaml
relationships:
  # One-to-Many
  - type: hasMany
    model: Post
    foreignKey: user_id      # Optional: custom foreign key

  # Many-to-One
  - type: belongsTo
    model: User
    foreignKey: user_id      # Optional: custom foreign key

  # One-to-One
  - type: hasOne
    model: Profile
    foreignKey: user_id

  # Many-to-Many
  - type: belongsToMany
    model: Role
    pivotTable: user_roles   # Optional: custom pivot table name

  # Polymorphic relationships
  - type: morphTo
    morphName: commentable   # Required: morph name

  - type: morphOne
    model: Image
    morphName: imageable

  - type: morphMany
    model: Comment
    morphName: commentable

  - type: morphToMany
    model: Tag
    morphName: taggable
    pivotTable: taggables    # Optional: custom pivot table
```

### Pivot Tables

Define pivot tables for many-to-many relationships:

```yaml
models:
  - name: User
    # ... user fields
    pivotTables:
      - name: user_roles
        model1: User
        model2: Role
        foreignKey1: user_id
        foreignKey2: role_id
        timestamps: true     # Optional: add timestamps to pivot
        additionalFields:    # Optional: extra fields in pivot table
          - name: assigned_at
            type: timestamp
          - name: assigned_by
            type: bigInteger
```

### Complete Example

```yaml
# Laravel E-commerce Models Configuration
outputDir: "."
namespace: "App\\Models"

models:
  - name: User
    table: users
    timestamps: true
    softDeletes: false
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
        index: true
      - name: email_verified_at
        type: timestamp
        nullable: true
      - name: password
        type: string
        length: 255
        nullable: false
      - name: role
        type: enum
        enumValues:
          - value: "admin"
            label: "Administrator"
          - value: "customer"
            label: "Customer"
        default: "customer"
    relationships:
      - type: hasMany
        model: Order
        foreignKey: user_id
      - type: belongsToMany
        model: Role
        pivotTable: user_roles
    pivotTables:
      - name: user_roles
        model1: User
        model2: Role
        foreignKey1: user_id
        foreignKey2: role_id
        timestamps: true

  - name: Product
    table: products
    timestamps: true
    softDeletes: true
    fields:
      - name: name
        type: string
        length: 255
        nullable: false
      - name: description
        type: text
        nullable: true
      - name: price
        type: decimal
        decimalPrecision:
          precision: 10
          scale: 2
        nullable: false
      - name: stock_quantity
        type: integer
        unsigned: true
        default: "0"
      - name: is_active
        type: boolean
        default: "true"
    relationships:
      - type: hasMany
        model: OrderItem
        foreignKey: product_id
      - type: morphMany
        model: Image
        morphName: imageable

  - name: Order
    table: orders
    timestamps: true
    fields:
      - name: user_id
        type: bigInteger
        nullable: false
        index: true
      - name: total_amount
        type: decimal
        decimalPrecision:
          precision: 10
          scale: 2
        nullable: false
      - name: status
        type: enum
        enumValues:
          - value: "pending"
          - value: "processing"
          - value: "shipped"
          - value: "delivered"
          - value: "cancelled"
        default: "pending"
    relationships:
      - type: belongsTo
        model: User
        foreignKey: user_id
      - type: hasMany
        model: OrderItem
        foreignKey: order_id

  - name: OrderItem
    table: order_items
    timestamps: true
    fields:
      - name: order_id
        type: bigInteger
        nullable: false
        index: true
      - name: product_id
        type: bigInteger
        nullable: false
        index: true
      - name: quantity
        type: integer
        unsigned: true
        nullable: false
      - name: price
        type: decimal
        decimalPrecision:
          precision: 10
          scale: 2
        nullable: false
    relationships:
      - type: belongsTo
        model: Order
        foreignKey: order_id
      - type: belongsTo
        model: Product
        foreignKey: product_id

  - name: Image
    table: images
    timestamps: true
    fields:
      - name: imageable_type
        type: string
        length: 255
        nullable: false
      - name: imageable_id
        type: bigInteger
        nullable: false
      - name: filename
        type: string
        length: 255
        nullable: false
      - name: path
        type: string
        length: 500
        nullable: false
      - name: alt_text
        type: string
        length: 255
        nullable: true
    relationships:
      - type: morphTo
        morphName: imageable
```

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

```yaml
models:
  - name: User
    table: users
    fields:
      - name: name
        type: string
      - name: email
        type: string
        unique: true
```

### 2. Use Descriptive Field Names
Field names help generate better fake data:

```yaml
fields:
  - name: first_name    # Generates fake()->firstName()
    type: string
  - name: email_address # Generates fake()->email()
    type: string
  - name: phone_number  # Generates fake()->phoneNumber()
    type: string
```

### 3. Organize Large Projects
Split large configurations into multiple files:

```bash
# Generate user-related models
schemly --config users.yml --only-models

# Generate product-related models
schemly --config products.yml --only-models

# Generate all migrations at once
schemly --config users.yml --config products.yml --only-migrations
```

### 4. Test Before Committing
Always test generated code:

```bash
# Generate in a test directory first
schemly --config models.yml --output /tmp/test-laravel

# Review generated files
ls -la /tmp/test-laravel/app/Models/

# Then generate in your real project
schemly --config models.yml --output /path/to/real-project
```

### 5. Version Control Integration
Add schemly to your development workflow:

```bash
# Generate fresh files
schemly --config models.yml --force

# Review changes
git diff

# Commit if satisfied
git add .
git commit -m "Regenerate Laravel models from YAML config"
```

## Troubleshooting

### Common Issues

**Permission Denied**
```bash
# Fix file permissions
chmod 755 /path/to/laravel-project
chmod -R 644 /path/to/laravel-project/app/Models/
```

**Invalid YAML**
```bash
# Validate YAML syntax
python -c "import yaml; yaml.safe_load(open('models.yml'))"
```

**Missing Directories**
```bash
# Ensure Laravel directory structure exists
mkdir -p app/Models database/migrations app/Http/Controllers
```

### Debug Mode

For detailed error information, use Rust's debug output:

```bash
RUST_BACKTRACE=1 schemly --config models.yml
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

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

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.

## Support

- 📖 [Documentation](https://github.com/DevPlus31/schemly/wiki)
- 🐛 [Issue Tracker](https://github.com/DevPlus31/schemly/issues)
- 💬 [Discussions](https://github.com/DevPlus31/schemly/discussions)

---

**Made with ❤️ and ☕ by developers, for developers.**