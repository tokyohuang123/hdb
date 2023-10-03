use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use std::path::Path;
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct KVStore {
    data: HashMap<String, String>,
}

impl KVStore {
    fn ensure_directory_exists(path: &str) {
        let path = Path::new(path);
        if !path.exists() {
            let parent_dir = path.parent().unwrap(); // 获取父目录
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    pub fn new() -> Self {
         // 加载环境变量
         dotenv().ok();

         // 从环境变量中获取存储路径
         let storage_path = env::var("STORAGE_PATH").expect("STORAGE_PATH not set in .env");
 
         // 确保存储目录存在
         KVStore::ensure_directory_exists(&storage_path);
        KVStore {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.data)?;
        fs::write(filename, json)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filename)?;
        let data: HashMap<String, String> = serde_json::from_str(&json)?;
        Ok(KVStore { data })
    }
}
