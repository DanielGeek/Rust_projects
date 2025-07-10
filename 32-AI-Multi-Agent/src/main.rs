use ai_multi_agent::run;


fn main() {
    let result = run();

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }
}
