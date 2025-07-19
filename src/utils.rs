use crate::error::{GeneratorError, Result};
use std::path::Path;

/// String manipulation utilities
pub mod string_utils {
    /// Convert string to snake_case
    pub fn to_snake_case(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch.is_uppercase() {
                if !result.is_empty() && chars.peek().map_or(false, |c| c.is_lowercase()) {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap());
            } else {
                result.push(ch);
            }
        }
        
        result
    }
    
    /// Convert string to camelCase
    pub fn to_camel_case(input: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for ch in input.chars() {
            if ch == '_' || ch == '-' || ch.is_whitespace() {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(ch.to_lowercase().next().unwrap());
            }
        }
        
        result
    }
    
    /// Convert string to PascalCase
    pub fn to_pascal_case(input: &str) -> String {
        let camel = to_camel_case(input);
        if let Some(first_char) = camel.chars().next() {
            format!("{}{}", first_char.to_uppercase(), &camel[1..])
        } else {
            camel
        }
    }
    
    /// Convert string to kebab-case
    pub fn to_kebab_case(input: &str) -> String {
        to_snake_case(input).replace('_', "-")
    }
    
    /// Pluralize a word (simple English rules)
    pub fn pluralize(word: &str) -> String {
        if word.is_empty() {
            return word.to_string();
        }
        
        let word_lower = word.to_lowercase();
        
        if word_lower.ends_with("y") && !word_lower.ends_with("ay") && !word_lower.ends_with("ey") && !word_lower.ends_with("iy") && !word_lower.ends_with("oy") && !word_lower.ends_with("uy") {
            format!("{}ies", &word[..word.len()-1])
        } else if word_lower.ends_with("s") || word_lower.ends_with("sh") || word_lower.ends_with("ch") || word_lower.ends_with("x") || word_lower.ends_with("z") {
            format!("{}es", word)
        } else if word_lower.ends_with("f") {
            format!("{}ves", &word[..word.len()-1])
        } else if word_lower.ends_with("fe") {
            format!("{}ves", &word[..word.len()-2])
        } else {
            format!("{}s", word)
        }
    }
    
    /// Singularize a word (simple English rules)
    pub fn singularize(word: &str) -> String {
        if word.is_empty() {
            return word.to_string();
        }
        
        let word_lower = word.to_lowercase();
        
        if word_lower.ends_with("ies") && word.len() > 3 {
            format!("{}y", &word[..word.len()-3])
        } else if word_lower.ends_with("ves") && word.len() > 3 {
            if word_lower.ends_with("ives") {
                format!("{}ife", &word[..word.len()-4])
            } else {
                format!("{}f", &word[..word.len()-3])
            }
        } else if word_lower.ends_with("ses") || word_lower.ends_with("shes") || word_lower.ends_with("ches") || word_lower.ends_with("xes") || word_lower.ends_with("zes") {
            if word_lower.ends_with("xes") && word.len() > 3 {
                &word[..word.len()-2]
            } else if word_lower.ends_with("ses") && word.len() > 3 {
                &word[..word.len()-2]
            } else {
                &word[..word.len()-2]
            }.to_string()
        } else if word_lower.ends_with("s") && word.len() > 1 {
            word[..word.len()-1].to_string()
        } else {
            word.to_string()
        }
    }
    

}

/// File system utilities
pub mod fs_utils {
    use super::*;
    
    /// Ensure directory exists, creating it if necessary
    pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(path)
                .map_err(|e| GeneratorError::Io(e))?;
        }
        Ok(())
    }
    

}

/// Collection utilities
pub mod collection_utils {
    use std::collections::HashMap;
    use std::hash::Hash;
    
    /// Group items by a key function
    pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
    where
        K: Eq + Hash,
        F: Fn(&T) -> K,
    {
        let mut groups = HashMap::new();
        for item in items {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }
        groups
    }
    
    /// Partition items into two groups based on predicate
    pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        
        for item in items {
            if predicate(&item) {
                true_items.push(item);
            } else {
                false_items.push(item);
            }
        }
        
        (true_items, false_items)
    }
    
    /// Find duplicates in a collection
    pub fn find_duplicates<T, K, F>(items: &[T], key_fn: F) -> Vec<K>
    where
        K: Eq + Hash + Clone,
        F: Fn(&T) -> K,
    {
        let mut seen = HashMap::new();
        let mut duplicates = Vec::new();
        
        for item in items {
            let key = key_fn(item);
            let count = seen.entry(key.clone()).or_insert(0);
            *count += 1;
            if *count == 2 {
                duplicates.push(key);
            }
        }
        
        duplicates
    }
}

/// Validation utilities
pub mod validation_utils {
    /// Check if string is a valid PHP identifier
    pub fn is_valid_php_identifier(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }
        
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }
    
    /// Check if string is a valid namespace
    pub fn is_valid_namespace(namespace: &str) -> bool {
        if namespace.is_empty() {
            return false;
        }
        
        namespace.split('\\').all(|part| is_valid_php_identifier(part))
    }
    
    /// Check if email is valid (basic check)
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_string_conversions() {
        assert_eq!(string_utils::to_snake_case("CamelCase"), "camel_case");
        assert_eq!(string_utils::to_camel_case("snake_case"), "snakeCase");
        assert_eq!(string_utils::to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(string_utils::to_kebab_case("CamelCase"), "camel-case");
    }

    #[test]
    fn test_pluralization() {
        assert_eq!(string_utils::pluralize("user"), "users");
        assert_eq!(string_utils::pluralize("category"), "categories");
        assert_eq!(string_utils::pluralize("box"), "boxes");
        assert_eq!(string_utils::pluralize("knife"), "knives");
        
        assert_eq!(string_utils::singularize("users"), "user");
        assert_eq!(string_utils::singularize("categories"), "category");
        assert_eq!(string_utils::singularize("boxes"), "box");
    }



    #[test]
    fn test_fs_utils() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");

        assert!(fs_utils::ensure_dir_exists(&test_dir).is_ok());
        assert!(test_dir.exists());
    }

    #[test]
    fn test_validation_utils() {
        assert!(validation_utils::is_valid_php_identifier("validName"));
        assert!(validation_utils::is_valid_php_identifier("_private"));
        assert!(!validation_utils::is_valid_php_identifier("123invalid"));
        assert!(!validation_utils::is_valid_php_identifier("invalid-name"));
        
        assert!(validation_utils::is_valid_namespace("App\\Models"));
        assert!(!validation_utils::is_valid_namespace("App\\123Invalid"));
        
        assert!(validation_utils::is_valid_email("test@example.com"));
        assert!(!validation_utils::is_valid_email("invalid-email"));
    }

    #[test]
    fn test_collection_utils() {
        let items = vec!["apple", "banana", "apricot", "blueberry"];
        let grouped = collection_utils::group_by(items, |s| s.chars().next().unwrap());
        
        assert_eq!(grouped.get(&'a').unwrap().len(), 2);
        assert_eq!(grouped.get(&'b').unwrap().len(), 2);
        
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = collection_utils::partition(numbers, |n| n % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}
