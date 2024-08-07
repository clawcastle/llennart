use clap::Parser;
use config::Config;
use rand::{Rng, RngCore};

mod config;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    question: String
}

fn main() -> anyhow::Result<()> {
    let config = Config::from_file("./data/config.json")?;

    let args = Args::parse();

    let llm_client = StubLlm;

    let question = Question::new(&args.question);

    let answer = llm_client.ask_question(&question);

    println!("[User] asked ({}):", &question.id);
    println!("{}\n", &question.question);

    println!("[{}] answered ({}):", &config.agent_name, &answer.id);
    println!("{}\n", &answer.answer);

    Ok(())
}

trait Llm {
    fn ask_question(&self, question: &Question) -> Answer;
}

struct StubLlm;

impl Llm for StubLlm {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_question_to_llm_client() {
        let llm_client = StubLlm;

        let answer = llm_client.ask_question(&Question::new("question"));

        println!("{}", &answer.answer);
    }

}