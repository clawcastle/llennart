use clap::Parser;
use rand::{Rng, RngCore};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    question: String
}

fn main() {
    let args = Args::parse();

    let llm_client = LlmClientStub;

    let question = Question::new(&args.question);

    let answer = llm_client.ask_question(&question);

    println!("Question asked (id: {}): '{}'.", &question.id, &question.question);

    println!("Answer (id: {}): \n", &answer.id);
    println!("{}\n", &answer.answer);
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
        let mut rand = rand::thread_rng();

        let id = rand.next_u64().to_string();

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
        let mut rand = rand::thread_rng();

        let id = rand.next_u64().to_string();

        Self {
            id,
            answer: answer.to_string()
        }
    }
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