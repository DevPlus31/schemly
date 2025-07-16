# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of schemly Laravel code generator
- YAML-based model configuration
- Complete Laravel component generation (models, controllers, resources, factories, migrations, pivot tables)
- File overwrite protection with `--force` flag
- Selective generation with `--only-*` and `--no-*` flags
- Comprehensive relationship support (all Laravel relationship types)
- Polymorphic relationship support
- Pivot table generation for many-to-many relationships
- Enhanced CLI with examples and safety information
- Detailed generation statistics and summaries
- Field validation and error handling
- Support for all Laravel field types including enums
- Intelligent fake data generation based on field names
- Comprehensive documentation and examples

### Features
- **Safe by Default**: Won't overwrite existing files unless `--force` is used
- **Selective Generation**: Choose exactly which components to generate
- **Full Laravel Support**: All relationship types and field types supported
- **Smart Validation**: Validates configuration before generation
- **Clear Feedback**: Detailed statistics and error messages
- **Fast Performance**: Built with Rust for maximum speed

### Supported Laravel Components
- ✅ Eloquent Models with relationships and casts
- ✅ HTTP Controllers with resource methods
- ✅ API Resources for JSON responses
- ✅ Model Factories with intelligent fake data
- ✅ Database Migrations with all field types
- ✅ Pivot Table Migrations for many-to-many relationships

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

## [0.1.0] - 2024-07-16

### Added
- Initial project setup
- Basic YAML parsing and validation
- Core generator architecture
- Model generation with basic field support
- Migration generation
- CLI interface with clap

---

## Future Roadmap

### Planned Features
- [ ] Custom validation rules generation
- [ ] Seeder generation
- [ ] Request validation classes
- [ ] API documentation generation
- [ ] GraphQL schema generation
- [ ] Database view support
- [ ] Custom template support
- [ ] Configuration file validation
- [ ] Interactive configuration builder

### Potential Enhancements
- [ ] Custom namespace configuration
- [ ] Batch processing of multiple config files
- [ ] IDE plugin support
- [ ] Web-based configuration editor
- [ ] Docker container support
- [ ] CI/CD integration examples