use serde::Deserialize;

pub struct Config {
    pub agent_name: String,
    pub models: Vec<ModelConfigEntry>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ModelConfigEntry {
    Stub { name: String },
    OpenAiGpt35 { name: String, url: String, api_key: String }
}

impl Config {
    pub fn from_file(file_path: &str) -> anyhow::Result<Self> {
        let file_content = std::fs::read_to_string(file_path)?;

        let config_json: ConfigFile = serde_json::from_str(&file_content)?;

        Ok(Self {
                    agent_name: config_json.agent_name.unwrap_or(String::from("Llennart")),
                    models: config_json.models
                })
    }
}

#[derive(Deserialize)]
struct ConfigFile {
    agent_name: Option<String>,
    models: Vec<ModelConfigEntry>
}

#[cfg(test)]
mod tests {
    use crate::config::ModelConfigEntry;

    use super::ConfigFile;

    #[test]
    fn can_parse_valid_config_file() {
        let config_file_json = 
            "{ \"agent_name\": \"agent-name\", \"llm_api_key\": \"api-key\", \"llm_url\": \"url\", \"llms\": [ { \"type\": \"Stub\", \"name\": \"Stub llm\" } ] }";

        let config_file_result: Result<ConfigFile, serde_json::Error> = serde_json::from_str(config_file_json);

        assert!(config_file_result.is_ok());
    }

    #[test]
    fn can_parse_config_with_multiple_different_llm_variants() {
        let config_file_json = 
        "{ \"agent_name\": \"Lllenart 2\", \"llm_api_key\": \"api key\", \"llm_url\": \"url\", \"llms\": [ { \"type\": \"Stub\", \"name\": \"Stub llm\" }, { \"type\": \"OpenAi\", \"name\": \"OpenAi llm\", \"url\": \"url.com\", \"api_key\": \"api-key\" } ] }";

        let config_file_result: Result<ConfigFile, serde_json::Error> = serde_json::from_str(config_file_json);

        assert!(config_file_result.is_ok());

        let parsed_config = config_file_result.unwrap();
        
        assert!(parsed_config.models.len() == 2);
        
        let has_stub_llm = parsed_config.models.iter().any(|x| matches!(x, ModelConfigEntry::Stub { name: _ }));
        let has_openai_llm = parsed_config.models.iter().any(|x| matches!(x, ModelConfigEntry::OpenAiGpt35 { name: _, url: _, api_key: _ }));

        assert!(has_stub_llm);
        assert!(has_openai_llm);
    }
}