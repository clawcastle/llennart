use clap::Parser;
use rand::{Rng, RngCore};

#[derive(Parser, Debug)]
struct Args {

}

fn main() {
    let args = Args::parse();
}

trait LlmClient {
    fn ask_question(&self, question: &str) -> String;
}

struct LlmClientStub;

impl LlmClient for LlmClientStub {
    fn ask_question(&self, question: &str) -> String {
        let mut rng = rand::thread_rng();

        let n: usize = rng.gen_range(20..200);

        let mut result = String::new();

        for i in 0..n {
            result.push_str("answer ");

            if i % 15 == 0 {
                result.push_str("\n\n");
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_question_to_llm_client() {
        let llm_client = LlmClientStub;

        let answer = llm_client.ask_question("");

        println!("{}", &answer);
    }

}