use std::fs;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandConfig {
    pub binary_path: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WatcherConfig {
    pub command_config: CommandConfig,
    pub debounce_in_seconds: u8,
    pub path_to_watch: String,
    pub file_extension_to_watch_for: Option<String>,
}

impl WatcherConfig {
    pub fn extension(&self) -> String {
        let extension = self.file_extension_to_watch_for.clone();
        let extension = extension.unwrap_or(".py".to_string());
        extension
    }
}

pub fn build_config(file_path: &str) -> WatcherConfig {
    let mut file = fs::File::open(file_path)
        .expect("!!fileopen");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("!!read_to_string");

    serde_json::from_str::<WatcherConfig>(buffer.as_str())
        .expect("!!json_from_str")
}
