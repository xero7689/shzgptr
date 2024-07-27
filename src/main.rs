use dotenvy::dotenv;
use std::env;
use clap::{App, Arg};

pub mod openai;

use crate::openai::OpenAIClient;

#[tokio::main]
async fn main() {
    /* Prepare API Key */
    dotenv().expect(".env file not found");
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    /* Parse CLI Arguments */
    let matches = App::new("shzgptr")
        .version("0.1.1")
        .author("SHZ Lee")
        .about("Rust Command Line GPT")
        .arg(
            Arg::with_name("message")
                .value_name("MESSAGE")
                .help("Input message you want to ask LLM")
                .required(true)
                .short("m"),
        )
        .get_matches();

    /* Query LLM */
    let user_message = matches.values_of_lossy("message").unwrap().join(" ");

    println!("User: {:?}", &user_message);

    let llm_client = OpenAIClient {
        api_key: openai_api_key,
        model_id: "gpt-4o-mini".into(),
    };

    let completion = llm_client.chat_completions(user_message).await.unwrap();
    let assistant_message = completion.choices[0].message.content.clone();

    println!("Assistant: {}", &assistant_message);
}
