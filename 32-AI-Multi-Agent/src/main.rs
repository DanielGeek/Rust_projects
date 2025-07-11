use ai_multi_agent::{config::Config, run};

fn main() {
    let config = Config::new().unwrap();

    match run(config) {
        Ok(_) => println!("Thanks for using AI Todo, please come again."),
        Err(error) => eprintln!("THere was an error using AI Todo :( {error}"),
    }
}
