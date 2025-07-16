pub mod model_generator;
pub mod migration_generator;
pub mod controller_generator;
pub mod resource_generator;
pub mod factory_generator;
pub mod pivot_table_generator;

use crate::error::Result;
use crate::types::{Config, ModelDefinition};

pub trait Generator {
    fn generate(&self, model: &ModelDefinition, config: &Config) -> Result<String>;
    fn get_file_path(&self, model: &ModelDefinition, config: &Config) -> String;
}

// Use the Generator trait for both models and pivot tables