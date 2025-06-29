use llm::{
    builder::LLMBuilder,
    chain::{LLMRegistryBuilder, MultiChainStepBuilder, MultiPromptChain},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gemma = LLMBuilder::new()
        .backend(llm::builder::LLMBackend::Ollama)
        .model("gemma3:1b")
        .build()?;

    let llama = LLMBuilder::new()
        .backend(llm::builder::LLMBackend::Ollama)
        .model("llama3.2:1b")
        .build()?;

    let registry = LLMRegistryBuilder::new()
        .register("gemma", gemma)
        .register("llama", llama)
        .build();

    let chain_res = MultiPromptChain::new(&registry)
        .step(
            MultiChainStepBuilder::new(llm::chain::MultiChainStepMode::Chat)
            .provider_id("gemma")
            .id("summarization")
            .template("Summarize the key features of rust programming language in 3 concise bullet point.")
            .build()?
        )
        .step(
            MultiChainStepBuilder::new(llm::chain::MultiChainStepMode::Chat)
            .provider_id("llama")
            .id("documentation")
            .template("Here is a summary: {{summarization}} \n\n Create a breif markdown documentation section explaining these points for a begineer")
            .build()?
        ).run()
        .await?;

    println!("Output: {:#?}", chain_res);

    Ok(())
}
