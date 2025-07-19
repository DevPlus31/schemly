# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2024-07-19

### 🎉 Major Release - Enhanced Examples & Documentation

This release significantly improves the developer experience with comprehensive examples, AI-friendly documentation, and critical bug fixes.

### ✨ Added
- **Three Complete Examples**: Production-ready YAML configurations for different use cases
  - `examples/linktree.yaml` - Simple social media link aggregator (3 models, 18 files)
  - `examples/ecommerce.yaml` - Full e-commerce platform (8 models, 48 files)
  - `examples/blog.yaml` - Advanced blog system with DDD structure (8 models, 48 files)
- **AI-Friendly Documentation**: Comprehensive YAML syntax guide in `docs/YAML_SYNTAX_GUIDE.md`
  - Complete field type reference with examples
  - All relationship patterns documented
  - Validation rules and best practices
  - AI prompt templates and usage guidelines
- **Domain-Driven Design Support**: Optional DDD folder structure
  - Models: `app/Domain/{ModelName}/Models/{ModelName}.php`
  - DTOs: `app/Domain/{ModelName}/DTOs/{ModelName}DTO.php`
  - Resources: `app/Domain/{ModelName}/Resources/{ModelName}Resource.php`
- **Enhanced Field Types**: Complete support for all Laravel field types
  - Decimal fields with precision and scale
  - Enum fields with value/label pairs
  - JSON fields with array casting
  - Polymorphic relationship fields

### 🐛 Fixed
- **Critical Field Naming Bug**: Fixed snake_case naming convention for YAML fields
  - `enumValues` → `enum_values` (Breaking Change)
  - `decimalPrecision` → `decimal_precision` (Breaking Change)
- **Polymorphic Relationship Parsing**: Fixed `morphTo` relationship parsing issues
  - Added required empty `model: ""` field for morphTo relationships
- **Enum Field Validation**: Improved validation for enum fields requiring values
- **Decimal Field Validation**: Enhanced validation for decimal precision requirements

### 📚 Documentation
- **Updated README.md**: Added version badge, examples section, and comprehensive changelog
- **Field Type Quick Reference**: Complete mapping table for YAML → Laravel → Database types
- **Relationship Patterns**: All Laravel relationship types with working examples
- **Common Mistakes Guide**: What to avoid and how to fix common issues
- **AI Usage Guidelines**: Best practices for AI-assisted code generation

### 🔧 Improved
- **Error Messages**: More descriptive validation errors with line numbers
- **Example Quality**: All examples tested and verified to generate successfully
- **Documentation Structure**: Better organization and searchability
- **Code Generation**: More robust handling of complex relationships

### ⚠️ Breaking Changes
- **Field Naming Convention**: YAML field names now use snake_case instead of camelCase
  - Update `enumValues` to `enum_values` in your YAML files
  - Update `decimalPrecision` to `decimal_precision` in your YAML files
- **Polymorphic Relationships**: `morphTo` relationships now require an empty `model: ""` field

### 📊 Statistics
- **Examples**: 3 complete, tested examples covering different complexity levels
- **Documentation**: 1000+ lines of comprehensive YAML syntax documentation
- **Field Types**: 26+ field types fully documented with examples
- **Relationships**: 8 relationship types with working examples
- **Generated Files**: Up to 48 Laravel files per example (models, controllers, resources, factories, migrations, DTOs)

## [0.1.0] - 2024-07-16

### Added
- Initial release of Schemly Laravel code generator
- YAML-based model configuration system
- Complete Laravel component generation:
  - ✅ Eloquent Models with relationships and casts
  - ✅ HTTP Controllers with resource methods
  - ✅ API Resources for JSON responses
  - ✅ Model Factories with intelligent fake data
  - ✅ Database Migrations with all field types
  - ✅ Pivot Table Migrations for many-to-many relationships
- File overwrite protection with `--force` flag
- Selective generation with `--only-*` and `--no-*` flags
- Comprehensive relationship support (all Laravel relationship types)
- Polymorphic relationship support
- Enhanced CLI with examples and safety information
- Detailed generation statistics and summaries
- Field validation and error handling
- Support for all Laravel field types including enums
- Intelligent fake data generation based on field names

### Features
- **Safe by Default**: Won't overwrite existing files unless `--force` is used
- **Selective Generation**: Choose exactly which components to generate
- **Full Laravel Support**: All relationship types and field types supported
- **Smart Validation**: Validates configuration before generation
- **Clear Feedback**: Detailed statistics and error messages
- **Fast Performance**: Built with Rust for maximum speed

### Supported Field Types
- String types: `string`, `text`, `longText`, `mediumText`
- Numeric types: `integer`, `bigInteger`, `tinyInteger`, `smallInteger`, `mediumInteger`, `float`, `decimal`
- Date types: `date`, `dateTime`, `timestamp`
- Special types: `boolean`, `json`, `uuid`, `enum`, `binary`, `inet`

### Supported Relationships
- One-to-Many: `hasMany`, `belongsTo`
- One-to-One: `hasOne`
- Many-to-Many: `belongsToMany`
- Polymorphic: `morphTo`, `morphOne`, `morphMany`, `morphToMany`

### Initial Architecture
- Core generator architecture with Rust
- YAML parsing and validation
- CLI interface with clap
- Template-based code generation
- Comprehensive error handling

---

## 🔄 Migration Guide

### Upgrading from 0.1.0 to 0.8.0

#### Breaking Changes

**1. Field Naming Convention**
```yaml
# ❌ Old (0.1.0) - camelCase
fields:
  - name: status
    type: enum
    enumValues:
      - value: "active"
        label: "Active"

  - name: price
    type: decimal
    decimalPrecision:
      precision: 10
      scale: 2

# ✅ New (0.8.0) - snake_case
fields:
  - name: status
    type: enum
    enum_values:
      - value: "active"
        label: "Active"

  - name: price
    type: decimal
    decimal_precision:
      precision: 10
      scale: 2
```

**2. Polymorphic Relationships**
```yaml
# ❌ Old (0.1.0) - Used camelCase
relationships:
  - type: morphTo
    morphName: commentable

# ✅ New (0.8.0) - Uses snake_case, no model field needed
relationships:
  - type: morphTo
    morph_name: commentable
```

#### Automated Migration
```bash
# Use sed to update field names in your YAML files
sed -i 's/enumValues:/enum_values:/g' your-config.yaml
sed -i 's/decimalPrecision:/decimal_precision:/g' your-config.yaml

# Add empty model field to morphTo relationships
# (Manual review recommended for this change)
```

---

## 🚀 Future Roadmap

### 🎯 Version 0.9.0 (Planned)
- [ ] **Custom Validation Rules**: Generate Laravel form request classes
- [ ] **Seeder Generation**: Database seeders with realistic test data
- [ ] **Request Classes**: Form request validation classes
- [ ] **Enhanced CLI**: Interactive mode for model selection
- [ ] **Configuration Validation**: Pre-generation YAML validation

### 🎯 Version 1.0.0 (Planned)
- [ ] **API Documentation**: Auto-generate OpenAPI/Swagger specs
- [ ] **GraphQL Support**: Generate GraphQL schema and resolvers
- [ ] **Custom Templates**: User-defined code generation templates
- [ ] **IDE Integration**: VS Code extension and PHPStorm plugin
- [ ] **Web Interface**: Browser-based configuration editor

### 🔮 Future Enhancements
- [ ] **Database Views**: Support for database view generation
- [ ] **Custom Namespaces**: Configurable namespace structures
- [ ] **Batch Processing**: Multiple configuration files at once
- [ ] **Docker Support**: Containerized generation environment
- [ ] **CI/CD Integration**: GitHub Actions and GitLab CI examples
- [ ] **Multi-Language**: Support for other PHP frameworks
- [ ] **Real-time Sync**: Watch mode for automatic regeneration

### 🤝 Community Features
- [ ] **Plugin System**: Third-party generator plugins
- [ ] **Template Marketplace**: Community-shared templates
- [ ] **Configuration Sharing**: Public configuration repository
- [ ] **Integration Examples**: Real-world project examples

---

## 📈 Version Comparison

| Feature | 0.1.0 | 0.8.0 | Planned 1.0.0 |
|---------|-------|-------|---------------|
| **Basic Generation** | ✅ | ✅ | ✅ |
| **Examples** | ❌ | ✅ (3) | ✅ (5+) |
| **Documentation** | Basic | ✅ AI-Friendly | ✅ Interactive |
| **Field Types** | Basic | ✅ Complete | ✅ Extended |
| **Relationships** | Basic | ✅ All Types | ✅ Advanced |
| **DDD Support** | ❌ | ✅ | ✅ Enhanced |
| **Validation** | Basic | ✅ Enhanced | ✅ Custom Rules |
| **CLI Interface** | Basic | ✅ Enhanced | ✅ Interactive |
| **Error Handling** | Basic | ✅ Detailed | ✅ Contextual |

---

## 🏆 Acknowledgments

### Contributors
- **DevPlus31** - Project creator and maintainer
- **Community** - Bug reports, feature requests, and feedback

### Special Thanks
- **Laravel Community** - For the amazing framework
- **Rust Community** - For the powerful language and ecosystem
- **YAML Specification** - For the human-readable configuration format

---

## 📞 Support & Community

- 🌐 **Website**: [https://schemly.dev/](https://schemly.dev/)
- 📖 **Documentation**: [GitHub Wiki](https://github.com/DevPlus31/schemly/wiki)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/DevPlus31/schemly/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/DevPlus31/schemly/discussions)
- 📧 **Email**: support@schemly.dev

**Made with ❤️ and ☕ by developers, for developers.**