use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tech {
    pub name: String,
    pub levels: Vec<String>,
    pub modules: Vec<String>,
    pub challenges: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub techs: HashMap<String, Tech>,
}

impl Database {
    /// Carrega o arquivo JSON de dados para a memória
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let db: Database = serde_json::from_str(&content)?;
        Ok(db)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_database() {
        let db = Database::load_from_file("data/trails.json");
        assert!(db.is_ok(), "O arquivo trails.json deve ser lido com sucesso");
        
        let database = db.unwrap();
        assert!(database.techs.contains_key("rust"), "A trilha de Rust deve existir");
    }
}