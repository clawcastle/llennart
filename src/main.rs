use std::error::Error;

use clap::Parser;
use rand::{Rng, RngCore};
use serde::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    question: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_file("./data/config.json")?;

    let args = Args::parse();

    let llm_client = LlmClientStub;

    let question = Question::new(&args.question);

    let answer = llm_client.ask_question(&question);

    println!("[User] asked ({}):", &question.id);
    println!("{}\n", &question.question);

    println!("[{}] answered ({}):", &config.agent_name, &answer.id);
    println!("{}\n", &answer.answer);

    Ok(())
}

trait LlmClient {
    fn ask_question(&self, question: &Question) -> Answer;
}

struct LlmClientStub;

impl LlmClient for LlmClientStub {
    fn ask_question(&self, question: &Question) -> Answer {
        let mut rng = rand::thread_rng();

        let n: usize = rng.gen_range(20..200);

        let mut result = String::new();

        for i in 0..n {
            result.push_str("answer ");

            if i % 15 == 0 {
                result.push_str("\n\n");
            }
        }
        
        Answer::new(&result)
    }
}

struct Question {
    id: String,
    question: String
}

impl Question {
    fn new(question: &str) -> Self {
        let id = generate_id();

        Self {
            id,
            question: question.to_string()
        }
    }
}

struct Answer {
    id: String,
    answer: String
}

impl Answer {
    fn new(answer: &str) -> Self {
        let id = generate_id();

        Self {
            id,
            answer: answer.to_string()
        }
    }
}

fn generate_id() -> String {
    let mut rand = rand::thread_rng();

    let id = rand.next_u64().to_string();

    id
}

struct Config {
    agent_name: String,
    llm_api_key: String,
    llm_url: String
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(file_path)?;

        let config_json: ConfigFile = serde_json::from_str(&file_content)?;

        Ok(Self {
                    agent_name: config_json.agent_name.unwrap_or(String::from("Llennart")),
                    llm_api_key: config_json.llm_api_key,
                    llm_url: config_json.llm_url
                })
    }
}

#[derive(Deserialize)]
struct ConfigFile {
    agent_name: Option<String>,
    llm_api_key: String,
    llm_url: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_question_to_llm_client() {
        let llm_client = LlmClientStub;

        let answer = llm_client.ask_question(&Question::new("question"));

        println!("{}", &answer.answer);
    }

}