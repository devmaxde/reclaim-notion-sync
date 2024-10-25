use std::fs;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Debug)]
pub enum SyncPriority{
    NOTION,
    RECLAIM
}




impl Serialize for SyncPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(match self {
            SyncPriority::NOTION => "Notion",
            SyncPriority::RECLAIM => "Reclaim",
        })
    }
}



#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfig{
    database_id: String,
    sync_priority: SyncPriority
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SyncConfig{
    reclaim_api_key: String,
    notion_api_key: String,
    databases: Vec<DatabaseConfig>
}

#[derive(Debug)]
pub enum SyncConfigError{
    FileNotFound,
    FileInvalid
}

impl SyncConfig {

    pub fn from_config_file(
    ) -> Result<SyncConfig, SyncConfigError> {
        let config_path = "./config.toml";
        let example_path = "./config_example.toml";

        let config_file = std::fs::read_to_string(config_path);
        if config_file.is_err() {
            SyncConfig::create_example_config_file(example_path.to_string(), true);
            return Err(SyncConfigError::FileNotFound);
        }
        let config_file = config_file.unwrap();
        let sync_config: SyncConfig = toml::from_str(&config_file).map_err(|_| SyncConfigError::FileInvalid)?;
        Ok(sync_config)
    }


    pub fn create_example_config_file(path: String, force_create: bool) {
        let wing_config = SyncConfig{
            reclaim_api_key: "RECLAIM_API_KEY".to_string(),
            notion_api_key: "NOTION_API_KEY".to_string(),
            databases: vec![DatabaseConfig{
                database_id: "DATABASE_01".to_string(),
                sync_priority: SyncPriority::NOTION,
            }, DatabaseConfig{
                database_id: "DATABASE_02".to_string(),
                sync_priority: SyncPriority::RECLAIM,
            }],
        };
        let toml = toml::to_string(&wing_config).unwrap();

        if force_create {
            if fs::metadata(path.clone()).is_ok() {
                // File exists, proceed to remove it
                fs::remove_file(path.clone()).unwrap();
            } else {
                // File does not exist, handle accordingly
                println!("File does not exist, no need to remove");
            }
        }

        if !std::path::Path::new(&path).exists() {
            fs::write(path, toml).unwrap();
        }
    }
}