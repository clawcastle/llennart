
use clap::Parser;
use config::Config;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

mod config;

const OPEN_AI_API_BASE_URL: &'static str = "https://api.openai.com/v1";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    question: String
}

fn main() -> anyhow::Result<()> {
    let config = Config::from_file("./data/config.json")?;

    let args = Args::parse();

    let llm: Box<dyn Llm> = match config.models.first().unwrap() {
        config::ModelConfigEntry::Stub { name: _ } => Box::new(StubLlm),
        config::ModelConfigEntry::OpenAi { name: _, model, api_key } => {
            let model = OpenAiModel::try_from(model.as_str()).unwrap();

            Box::new(OpenAiLlm { model, api_key: api_key.to_string() })
        },
    };

    let question = Question::new(&args.question);

    let answer = llm.ask_question(&question)?;

    println!("[User] asked ({}):", &question.id);
    println!("{}\n", &question.question);

    println!("[{}] answered ({}):", &config.agent_name, &answer.id);
    println!("{}\n", &answer.content);

    Ok(())
}

trait Llm {
    fn ask_question(&self, question: &Question) -> anyhow::Result<Answer>;
}

struct StubLlm;
struct OpenAiLlm {
    model: OpenAiModel,
    api_key: String
}

impl Llm for StubLlm {
    fn ask_question(&self, question: &Question) -> anyhow::Result<Answer> {
        let mut rng = rand::thread_rng();

        let n: usize = rng.gen_range(20..200);

        let mut result = String::new();

        for i in 0..n {
            result.push_str("answer ");

            if i % 15 == 0 {
                result.push_str("\n\n");
            }
        }
        
        Ok(Answer::new(&result))
    }
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiChatCompletionMessageRequest {
    pub role: String, // TODO: How to handle role
    pub content: String
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiChatCompletionRequest {
    pub model: String, // TODO: Enum
    pub messages: Vec<OpenAiChatCompletionMessageRequest>
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiChatCompletionChoiceResponse {
    pub message: OpenAiChatCompletionMessageResponse
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiChatCompletionMessageResponse {
    pub content: String
}


#[derive(Debug, Clone, Deserialize)]
struct OpenAiChatCompletionResponse {
    pub choices: Vec<OpenAiChatCompletionChoiceResponse>
}

enum OpenAiModel {
    Gpt4oMini
}

impl OpenAiModel {
    fn model_string(&self) -> &'static str {
        match self {
            OpenAiModel::Gpt4oMini => "gpt-4o-mini",
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct InvalidModelNameError;

impl TryFrom<&str> for OpenAiModel {
    type Error = InvalidModelNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "gpt-4o-mini" => Ok(Self::Gpt4oMini),
            _ => Err(InvalidModelNameError)
        }
    }
}

impl Llm for OpenAiLlm {
    fn ask_question(&self, question: &Question) -> anyhow::Result<Answer> {
        let request = OpenAiChatCompletionRequest {
            model: String::from(self.model.model_string()),
            messages: vec![
                OpenAiChatCompletionMessageRequest { role: String::from("system"), content: question.question.clone() }
            ]
        };

        let request_json = serde_json::to_string(&request)?;

        let client = reqwest::blocking::Client::new();

        let response = client.post(format!("{}//chat/completions", OPEN_AI_API_BASE_URL))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .body(request_json)
            .send()?;

        let status = response.status();

        if (status.is_success()) {
            let response_json: OpenAiChatCompletionResponse  = response.json()?;
            let answer_text = response_json.choices.first().map(|c| c.message.content.clone());
    
            match answer_text {
                Some(answer_text) => Ok(Answer { id: generate_id(), content: answer_text }),
                None => todo!(),
            }
        } else {
            let response_body = response.text()?;

            println!("Request failed with status {}.", &status);
            println!("Request failure body: {}", &response_body);

            todo!();
        }
    }
}

struct Question {
    pub id: String,
    pub question: String
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
    content: String
}

impl Answer {
    fn new(answer: &str) -> Self {
        let id = generate_id();

        Self {
            id,
            content: answer.to_string()
        }
    }
}

fn generate_id() -> String {
    let mut rand = rand::thread_rng();

    let id = rand.next_u64().to_string();

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_question_to_llm_client() {
        let llm_client = StubLlm;

        let answer = llm_client.ask_question(&Question::new("question"));

        println!("{}", &answer.unwrap().content);
    }
}