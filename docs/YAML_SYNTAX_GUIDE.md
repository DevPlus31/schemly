# Schemly YAML Syntax Guide

**AI-Friendly Documentation for Laravel Code Generation**

This document provides a comprehensive, structured guide to the YAML syntax used by Schemly for generating Laravel models, DTOs, controllers, and related components.

## 📋 Table of Contents

1. [Quick Reference](#quick-reference)
2. [Root Configuration](#root-configuration)
3. [Model Definition](#model-definition)
4. [Field Types](#field-types)
5. [Relationships](#relationships)
6. [Validation Rules](#validation-rules)
7. [Complete Examples](#complete-examples)
8. [AI Usage Guidelines](#ai-usage-guidelines)

---

## 🚀 Quick Reference

### Minimal YAML Structure
```yaml
models:
  - name: ModelName          # Required: PHP class name (PascalCase)
    table: table_name        # Required: Database table name (snake_case)
    fields:                  # Required: At least one field OR timestamps: true
      - name: field_name     # Required: Field name (snake_case)
        type: string         # Required: Field type (see Field Types section)
```

### Complete Structure Template
```yaml
# Global Configuration
outputDir: "./generated"                    # Optional: Output directory
namespace: "App\\Models"                    # Optional: Base namespace
generateModels: true                        # Optional: Generate model files
generateControllers: false                 # Optional: Generate controller files
generateResources: false                   # Optional: Generate resource files
generateFactories: false                   # Optional: Generate factory files
generateMigrations: false                  # Optional: Generate migration files
generatePivotTables: false                 # Optional: Generate pivot tables
generateValidationRules: false             # Optional: Generate validation rules
generateDto: false                         # Optional: Generate DTO classes
useDddStructure: false                     # Optional: Use Domain-Driven Design structure
databaseEngine: "mysql"                    # Optional: Database engine
forceOverwrite: false                      # Optional: Overwrite existing files

# Models Definition
models:
  - name: ModelName                         # Required: PHP class name
    table: table_name                       # Required: Database table name
    timestamps: true                        # Optional: Add created_at/updated_at
    softDeletes: false                      # Optional: Add deleted_at field
    fillableGuarded: all                    # Optional: all | fillable | guarded
    traits: []                              # Optional: Additional PHP traits
    
    # Fields Definition
    fields:
      - name: field_name                    # Required: Field name
        type: string                        # Required: Field type
        nullable: false                     # Optional: Allow NULL values
        unique: false                       # Optional: Unique constraint
        index: false                        # Optional: Database index
        length: 255                         # Optional: Field length
        default: "value"                    # Optional: Default value
        comment: "Field description"        # Optional: Field comment
        unsigned: false                     # Optional: Unsigned (numeric types)
        autoIncrement: false               # Optional: Auto increment
        primary: false                      # Optional: Primary key
        
        # Enum-specific options
        enum_values:                        # Optional: For enum type
          - value: "option1"
            label: "Option 1"
          - value: "option2"
            label: "Option 2"

        # Decimal-specific options
        decimal_precision:                  # Optional: For decimal type
          precision: 8
          scale: 2
        
        # Validation rules
        validationRules:                    # Optional: Laravel validation
          - rule: "required"
          - rule: "max"
            parameters: ["255"]
        
        # Model casting
        castType: "array"                   # Optional: Custom cast type
    
    # Relationships
    relationships:
      - type: hasMany                       # Required: Relationship type
        model: RelatedModel                 # Required: Related model name
        foreignKey: foreign_key_id          # Optional: Custom foreign key
        localKey: id                        # Optional: Custom local key
        pivotTable: pivot_table_name        # Optional: For many-to-many
        pivotFields: ["field1", "field2"]   # Optional: Additional pivot fields
        morphName: morphable                # Optional: For polymorphic
        onDelete: cascade                   # Optional: Foreign key constraint
        onUpdate: cascade                   # Optional: Foreign key constraint
        withTimestamps: false               # Optional: Pivot table timestamps
    
    # Model-level validation rules
    validationRules:                        # Optional: Model validation
      - rule: "required_if"
        parameters: ["other_field", "value"]
    
    # Pivot tables (for many-to-many relationships)
    pivotTables:
      - name: pivot_table_name              # Required: Pivot table name
        model1: FirstModel                  # Required: First model
        model2: SecondModel                 # Required: Second model
        foreignKey1: first_model_id         # Required: First foreign key
        foreignKey2: second_model_id        # Required: Second foreign key
        timestamps: false                   # Optional: Add timestamps
        additionalFields:                   # Optional: Extra pivot fields
          - name: extra_field
            type: string
```

---

## 🔧 Root Configuration

### Global Settings
```yaml
# Output Configuration
outputDir: "./app"                          # Where to generate files
namespace: "App\\Models"                    # Base PHP namespace

# Generation Flags (all optional, default: false except generateModels)
generateModels: true                        # Generate Eloquent models
generateControllers: false                 # Generate API controllers
generateResources: false                   # Generate API resources
generateFactories: false                   # Generate model factories
generateMigrations: false                  # Generate database migrations
generatePivotTables: false                 # Generate pivot table migrations
generateValidationRules: false             # Generate form request classes
generateDto: false                         # Generate Data Transfer Objects

# Structure Options
useDddStructure: false                     # Use Domain-Driven Design folders
databaseEngine: "mysql"                    # Database engine (mysql, postgresql, sqlite)
forceOverwrite: false                      # Overwrite existing files without prompt
```

### DDD Structure Impact
When `useDddStructure: true`:
- Models: `app/Domain/{ModelName}/Models/{ModelName}.php`
- DTOs: `app/Domain/{ModelName}/DTOs/{ModelName}DTO.php`
- Resources: `app/Domain/{ModelName}/Resources/{ModelName}Resource.php`

When `useDddStructure: false` (traditional Laravel):
- Models: `app/Models/{ModelName}.php`
- DTOs: `app/DTOs/{ModelName}DTO.php`
- Resources: `app/Http/Resources/{ModelName}Resource.php`

---

## 📊 Model Definition

### Required Fields
```yaml
models:
  - name: User              # REQUIRED: PHP class name (PascalCase)
    table: users            # REQUIRED: Database table name (snake_case)
    fields: []              # REQUIRED: Must have fields OR timestamps: true
```

### Optional Model Properties
```yaml
models:
  - name: User
    table: users
    timestamps: true                        # Add created_at, updated_at (default: false)
    softDeletes: false                     # Add deleted_at field (default: false)
    fillableGuarded: all                   # Options: all, fillable, guarded
    traits: ["HasFactory", "Notifiable"]   # Additional PHP traits
    
    # Fillable/Guarded Options
    fillableGuarded: fillable              # Use $fillable array
    # OR
    fillableGuarded: guarded               # Use $guarded array
    # OR  
    fillableGuarded: all                   # Allow mass assignment for all fields
```

---

## 🏗️ Field Types

### String Types
```yaml
fields:
  - name: title
    type: string            # VARCHAR with length
    length: 255             # Optional: default 255
    
  - name: description
    type: text              # TEXT column
    
  - name: content
    type: longText          # LONGTEXT column
    
  - name: summary
    type: mediumText        # MEDIUMTEXT column
```

### Numeric Types
```yaml
fields:
  - name: age
    type: integer           # INT
    unsigned: true          # Optional: UNSIGNED
    
  - name: user_id
    type: bigInteger        # BIGINT
    
  - name: count
    type: tinyInteger       # TINYINT (-128 to 127)
    
  - name: quantity
    type: smallInteger      # SMALLINT
    
  - name: large_number
    type: mediumInteger     # MEDIUMINT
    
  - name: rating
    type: float             # FLOAT
    
  - name: price
    type: decimal           # DECIMAL with precision
    decimal_precision:
      precision: 8          # Total digits
      scale: 2              # Decimal places
```

### Date/Time Types
```yaml
fields:
  - name: birth_date
    type: date              # DATE (Y-m-d)
    
  - name: created_at
    type: dateTime          # DATETIME (Y-m-d H:i:s)
    
  - name: updated_at
    type: timestamp         # TIMESTAMP
```

### Special Types
```yaml
fields:
  - name: is_active
    type: boolean           # BOOLEAN (0/1)
    default: "true"
    
  - name: settings
    type: json              # JSON column
    castType: "array"       # Cast to array in model
    
  - name: uuid
    type: uuid              # UUID column
    
  - name: status
    type: enum              # ENUM column
    enum_values:
      - value: "active"
        label: "Active"
      - value: "inactive"
        label: "Inactive"
    default: "active"
    
  - name: file_data
    type: binary            # BINARY data
    
  - name: ip_address
    type: inet              # IP address (PostgreSQL)
```

### Field Constraints
```yaml
fields:
  - name: email
    type: string
    length: 255
    nullable: false         # NOT NULL (default: false)
    unique: true           # UNIQUE constraint (default: false)
    index: true            # Database index (default: false)
    default: "guest@example.com"  # Default value
    comment: "User email address"  # Column comment
```

---

## 🔗 Relationships

### One-to-Many (hasMany)
```yaml
relationships:
  - type: hasMany
    model: Post             # Related model name
    foreignKey: user_id     # Optional: custom foreign key (default: {model}_id)
    localKey: id           # Optional: custom local key (default: id)
```

### Many-to-One (belongsTo)
```yaml
relationships:
  - type: belongsTo
    model: User
    foreignKey: user_id     # Optional: custom foreign key
    ownerKey: id           # Optional: custom owner key
    onDelete: cascade      # Optional: cascade, restrict, set null
    onUpdate: cascade      # Optional: cascade, restrict, set null
```

### One-to-One (hasOne)
```yaml
relationships:
  - type: hasOne
    model: Profile
    foreignKey: user_id
    localKey: id
```

### Many-to-Many (belongsToMany)
```yaml
relationships:
  - type: belongsToMany
    model: Role
    pivotTable: user_roles          # Optional: custom pivot table name
    foreignPivotKey: user_id        # Optional: custom foreign key
    relatedPivotKey: role_id        # Optional: custom related key
    pivotFields: ["assigned_at"]    # Optional: additional pivot fields
    withTimestamps: true           # Optional: pivot table timestamps
```

### Polymorphic Relationships
```yaml
# Polymorphic One-to-Many (morphMany)
relationships:
  - type: morphMany
    model: Comment
    morphName: commentable          # Required: morph name

# Polymorphic Many-to-One (morphTo)
relationships:
  - type: morphTo
    morphName: commentable          # Required: morph name

# Polymorphic One-to-One (morphOne)
relationships:
  - type: morphOne
    model: Image
    morphName: imageable

# Polymorphic Many-to-Many (morphToMany)
relationships:
  - type: morphToMany
    model: Tag
    morphName: taggable
    pivotTable: taggables          # Optional: custom pivot table
```

---

## ✅ Validation Rules

### Field-Level Validation
```yaml
fields:
  - name: email
    type: string
    validationRules:
      - rule: "required"
      - rule: "email"
      - rule: "max"
        parameters: ["255"]
      - rule: "unique"
        parameters: ["users", "email"]
```

### Model-Level Validation
```yaml
models:
  - name: User
    # ... other properties
    validationRules:
      - rule: "required_if"
        parameters: ["role", "admin"]
      - rule: "confirmed"        # For password confirmation
```

### Common Validation Rules
```yaml
validationRules:
  # Basic rules
  - rule: "required"
  - rule: "nullable"
  - rule: "string"
  - rule: "integer"
  - rule: "numeric"
  - rule: "boolean"
  - rule: "array"
  - rule: "email"
  - rule: "url"
  - rule: "date"
  - rule: "json"
  
  # Size constraints
  - rule: "min"
    parameters: ["3"]
  - rule: "max"
    parameters: ["255"]
  - rule: "between"
    parameters: ["1", "100"]
  - rule: "size"
    parameters: ["10"]
  
  # Pattern matching
  - rule: "regex"
    parameters: ["/^[A-Za-z]+$/"]
  - rule: "alpha"
  - rule: "alpha_num"
  - rule: "alpha_dash"
  
  # Database constraints
  - rule: "unique"
    parameters: ["table_name", "column_name"]
  - rule: "exists"
    parameters: ["table_name", "column_name"]
  
  # Conditional rules
  - rule: "required_if"
    parameters: ["other_field", "value"]
  - rule: "required_unless"
    parameters: ["other_field", "value"]
  - rule: "required_with"
    parameters: ["other_field"]
  - rule: "required_without"
    parameters: ["other_field"]
```

---

## 📋 Complete Examples

### Simple Blog System
```yaml
outputDir: "./app"
namespace: "App\\Models"
generateModels: true
generateDto: true
useDddStructure: false

models:
  # User Model
  - name: User
    table: users
    timestamps: true
    softDeletes: false
    fields:
      - name: name
        type: string
        length: 255
        nullable: false
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "max"
            parameters: ["255"]
      
      - name: email
        type: string
        length: 255
        nullable: false
        unique: true
        index: true
        validationRules:
          - rule: "required"
          - rule: "email"
          - rule: "unique"
            parameters: ["users", "email"]
      
      - name: password
        type: string
        length: 255
        nullable: false
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "min"
            parameters: ["8"]
      
      - name: role
        type: enum
        enum_values:
          - value: "admin"
            label: "Administrator"
          - value: "editor"
            label: "Editor"
          - value: "user"
            label: "User"
        default: "user"
        validationRules:
          - rule: "required"
          - rule: "in"
            parameters: ["admin", "editor", "user"]
    
    relationships:
      - type: hasMany
        model: Post
        foreignKey: user_id

  # Post Model
  - name: Post
    table: posts
    timestamps: true
    softDeletes: true
    fields:
      - name: user_id
        type: bigInteger
        nullable: false
        index: true
        validationRules:
          - rule: "required"
          - rule: "exists"
            parameters: ["users", "id"]
      
      - name: title
        type: string
        length: 255
        nullable: false
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "max"
            parameters: ["255"]
      
      - name: slug
        type: string
        length: 255
        nullable: false
        unique: true
        index: true
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "max"
            parameters: ["255"]
          - rule: "unique"
            parameters: ["posts", "slug"]
      
      - name: content
        type: longText
        nullable: false
        validationRules:
          - rule: "required"
          - rule: "string"
      
      - name: excerpt
        type: text
        nullable: true
        validationRules:
          - rule: "nullable"
          - rule: "string"
      
      - name: status
        type: enum
        enum_values:
          - value: "draft"
            label: "Draft"
          - value: "published"
            label: "Published"
          - value: "archived"
            label: "Archived"
        default: "draft"
        validationRules:
          - rule: "required"
          - rule: "in"
            parameters: ["draft", "published", "archived"]
      
      - name: published_at
        type: timestamp
        nullable: true
        validationRules:
          - rule: "nullable"
          - rule: "date"
      
      - name: view_count
        type: integer
        unsigned: true
        default: "0"
        validationRules:
          - rule: "integer"
          - rule: "min"
            parameters: ["0"]
      
      - name: metadata
        type: json
        nullable: true
        castType: "array"
        validationRules:
          - rule: "nullable"
          - rule: "array"
    
    relationships:
      - type: belongsTo
        model: User
        foreignKey: user_id
        onDelete: cascade
      
      - type: belongsToMany
        model: Tag
        pivotTable: post_tags
        withTimestamps: true

  # Tag Model
  - name: Tag
    table: tags
    timestamps: true
    fields:
      - name: name
        type: string
        length: 100
        nullable: false
        unique: true
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "max"
            parameters: ["100"]
          - rule: "unique"
            parameters: ["tags", "name"]
      
      - name: slug
        type: string
        length: 100
        nullable: false
        unique: true
        index: true
        validationRules:
          - rule: "required"
          - rule: "string"
          - rule: "max"
            parameters: ["100"]
          - rule: "unique"
            parameters: ["tags", "slug"]
    
    relationships:
      - type: belongsToMany
        model: Post
        pivotTable: post_tags
        withTimestamps: true

# Pivot table definition
pivotTables:
  - name: post_tags
    model1: Post
    model2: Tag
    foreignKey1: post_id
    foreignKey2: tag_id
    timestamps: true
```

---

## 🤖 AI Usage Guidelines

### For AI Assistants

When generating YAML configurations:

1. **Always include required fields**: `name`, `table`, and either `fields` or `timestamps: true`
2. **Use proper naming conventions**:
   - Model names: PascalCase (e.g., `User`, `BlogPost`)
   - Table names: snake_case plural (e.g., `users`, `blog_posts`)
   - Field names: snake_case (e.g., `user_id`, `created_at`)
3. **Validate field types** against the supported list
4. **Include appropriate validation rules** for each field
5. **Consider relationships** between models
6. **Use sensible defaults** for optional properties

### Common Patterns

```yaml
# ID fields (auto-generated, don't define manually)
# Laravel automatically adds: id (primary key, auto-increment)

# Foreign key pattern
- name: user_id
  type: bigInteger
  nullable: false
  index: true

# Enum pattern
- name: status
  type: enum
  enum_values:
    - value: "active"
      label: "Active"
    - value: "inactive"
      label: "Inactive"
  default: "active"

# Timestamp pattern (use timestamps: true instead of manual fields)
timestamps: true  # Adds created_at and updated_at automatically

# Soft delete pattern
softDeletes: true  # Adds deleted_at field automatically

# JSON field pattern
- name: metadata
  type: json
  nullable: true
  castType: "array"

# Decimal money pattern
- name: price
  type: decimal
  decimal_precision:
    precision: 8
    scale: 2
  unsigned: true
```

### Validation Guidelines

- Always add `required` rule for non-nullable fields
- Add `nullable` rule for nullable fields
- Include appropriate type validation (`string`, `integer`, `email`, etc.)
- Add length constraints (`max`, `min`, `between`)
- Include database constraints (`unique`, `exists`)
- Use conditional validation when appropriate

### Error Prevention

- Don't manually define `id` fields (auto-generated)
- Don't manually define `created_at`/`updated_at` (use `timestamps: true`)
- Don't manually define `deleted_at` (use `softDeletes: true`)
- Ensure foreign key fields match the related model's primary key type
- Validate enum values match the defined options
- Check that relationship models exist in the configuration

---

## 🎯 AI Prompt Templates

### Generate Basic Model
```
Create a YAML configuration for a Laravel model with the following requirements:
- Model name: [ModelName]
- Fields: [list of fields with types]
- Relationships: [describe relationships]
- Include appropriate validation rules
- Use Laravel best practices
```

### Generate E-commerce Models
```
Generate YAML for an e-commerce system with:
- User model (authentication)
- Product model (with categories)
- Order model (with order items)
- Category model
- Include all necessary relationships and validations
```

### Generate Blog System
```
Create YAML for a blog system including:
- User model (authors)
- Post model (with tags and categories)
- Comment model (polymorphic)
- Tag and Category models
- Include soft deletes where appropriate
```

---

## 🔍 Field Type Quick Reference

| YAML Type | Laravel Migration | PHP Cast | Database Type |
|-----------|------------------|----------|---------------|
| `string` | `string()` | `string` | VARCHAR |
| `text` | `text()` | `string` | TEXT |
| `longText` | `longText()` | `string` | LONGTEXT |
| `integer` | `integer()` | `integer` | INT |
| `bigInteger` | `bigInteger()` | `integer` | BIGINT |
| `decimal` | `decimal()` | `float` | DECIMAL |
| `boolean` | `boolean()` | `boolean` | BOOLEAN |
| `date` | `date()` | `date` | DATE |
| `dateTime` | `dateTime()` | `datetime` | DATETIME |
| `timestamp` | `timestamp()` | `datetime` | TIMESTAMP |
| `json` | `json()` | `array` | JSON |
| `uuid` | `uuid()` | `string` | UUID |
| `enum` | `enum()` | `string` | ENUM |

---

## ⚠️ Common Mistakes to Avoid

### ❌ Don't Do This
```yaml
models:
  - name: user  # Wrong: should be PascalCase
    table: User  # Wrong: should be snake_case
    fields:
      - name: id  # Wrong: ID is auto-generated
        type: integer
        primary: true
      - name: created_at  # Wrong: use timestamps: true instead
        type: timestamp
```

### ✅ Do This Instead
```yaml
models:
  - name: User  # Correct: PascalCase
    table: users  # Correct: snake_case plural
    timestamps: true  # Correct: auto-generates created_at/updated_at
    fields:
      - name: name  # Correct: don't define id manually
        type: string
```

---

## 🧪 Testing Your YAML

### Validation Checklist
- [ ] All model names are PascalCase
- [ ] All table names are snake_case and plural
- [ ] All field names are snake_case
- [ ] No manual `id` fields defined
- [ ] Use `timestamps: true` instead of manual timestamp fields
- [ ] Foreign keys follow `{model}_id` pattern
- [ ] Enum values are properly defined
- [ ] Validation rules are appropriate for field types
- [ ] Relationships reference existing models

### Sample Validation Command
```bash
# Test your YAML configuration
schemly validate config.yml

# Generate with dry-run to check output
schemly generate config.yml --dry-run
```

---

## 📚 Advanced Examples

### Polymorphic Comments System
```yaml
models:
  # Comment model (polymorphic)
  - name: Comment
    table: comments
    timestamps: true
    fields:
      - name: content
        type: text
        nullable: false
      - name: commentable_id
        type: bigInteger
        nullable: false
        index: true
      - name: commentable_type
        type: string
        nullable: false
        index: true
    relationships:
      - type: morphTo
        morphName: commentable

  # Post model (can have comments)
  - name: Post
    table: posts
    timestamps: true
    fields:
      - name: title
        type: string
        nullable: false
    relationships:
      - type: morphMany
        model: Comment
        morphName: commentable

  # Product model (can have comments)
  - name: Product
    table: products
    timestamps: true
    fields:
      - name: name
        type: string
        nullable: false
    relationships:
      - type: morphMany
        model: Comment
        morphName: commentable
```

### Multi-tenant System
```yaml
models:
  # Tenant model
  - name: Tenant
    table: tenants
    timestamps: true
    fields:
      - name: name
        type: string
        nullable: false
      - name: domain
        type: string
        nullable: false
        unique: true
    relationships:
      - type: hasMany
        model: User
      - type: hasMany
        model: Project

  # User model (belongs to tenant)
  - name: User
    table: users
    timestamps: true
    fields:
      - name: tenant_id
        type: bigInteger
        nullable: false
        index: true
      - name: name
        type: string
        nullable: false
      - name: email
        type: string
        nullable: false
        index: true
    relationships:
      - type: belongsTo
        model: Tenant
      - type: hasMany
        model: Project

  # Project model (scoped to tenant)
  - name: Project
    table: projects
    timestamps: true
    fields:
      - name: tenant_id
        type: bigInteger
        nullable: false
        index: true
      - name: user_id
        type: bigInteger
        nullable: false
        index: true
      - name: name
        type: string
        nullable: false
    relationships:
      - type: belongsTo
        model: Tenant
      - type: belongsTo
        model: User
```

---

## 🔧 Configuration Best Practices

### Development vs Production
```yaml
# Development configuration
outputDir: "./app"
generateModels: true
generateControllers: true
generateResources: true
generateFactories: true
generateMigrations: true
generateDto: true
forceOverwrite: true  # Safe for development

# Production configuration
outputDir: "./generated"
generateModels: true
generateControllers: false  # Review manually
generateResources: false   # Review manually
generateFactories: false   # Not needed in production
generateMigrations: true
generateDto: false
forceOverwrite: false  # Prevent accidental overwrites
```

### DDD Structure Example
```yaml
useDddStructure: true
namespace: "App\\Domain"

models:
  - name: User
    # Generated files:
    # - app/Domain/User/Models/User.php
    # - app/Domain/User/DTOs/UserDTO.php
    # - app/Domain/User/Resources/UserResource.php
```

This comprehensive guide provides everything needed for AI assistants and developers to generate accurate, production-ready Laravel model configurations using Schemly's YAML syntax.
