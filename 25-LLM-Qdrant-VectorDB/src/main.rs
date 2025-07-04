use std::time::Duration;

use qdrant_client::{
    Payload, Qdrant, QdrantError,
    config::QdrantConfig,
    qdrant::{
        CreateCollectionBuilder, DeleteCollectionBuilder, PointId, PointStruct,
        ScalarQuantizationBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
    },
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: u64,
    text: String,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}

const OLLAMA_API_URL: &str = "http://localhost:11434/api/embed";
const OLLAMA_MODEL: &str = "llama3.2:1b";

#[tokio::main]
async fn main() -> Result<(), QdrantError> {
    let q_config = QdrantConfig {
        uri: String::from("http://localhost:6334"),
        timeout: Duration::from_secs(10),
        connect_timeout: Duration::from_secs(10),
        keep_alive_while_idle: false,
        api_key: None,
        compression: None,
        check_compatibility: false,
    };

    let qdrant_client = Qdrant::new(q_config)?;

    qdrant_client
        .delete_collection(DeleteCollectionBuilder::new("documents").build())
        .await?;

    let create_collection = CreateCollectionBuilder::new("documents").vectors_config(
        VectorParamsBuilder::new(2048, qdrant_client::qdrant::Distance::Cosine)
            .quantization_config(ScalarQuantizationBuilder::default()),
    );

    qdrant_client.create_collection(create_collection).await?;

    let documents = vec![
        Document {
            id: 1,
            text: "Rust is a systems programming language that is fast and memory-efficient.".to_string(),
        },
        Document {
            id: 2,
            text: "Python is a popular programming language for web development and data science.".to_string(),
        },
        Document {
            id: 3,
            text: "JavaScript is a versatile language commonly used for building interactive web applications.".to_string(),
        },
        Document {
            id: 4,
            text: "Go is known for its simplicity and strong support for concurrency.".to_string(),
        },
        Document {
            id: 5,
            text: "C++ is widely used in performance-critical applications like game engines and operating systems.".to_string(),
        },
        Document {
            id: 6,
            text: "TypeScript extends JavaScript with static typing for large-scale applications.".to_string(),
        },
        Document {
            id: 7,
            text: "Kotlin is a modern programming language that runs on the JVM and is used for Android development.".to_string(),
        },
    ];

    let embeddings = generate_embeddings(
        &documents
            .iter()
            .map(|d| d.text.as_str())
            .collect::<Vec<_>>(),
    )
    .await;

    let points: Vec<PointStruct> = documents
        .into_iter()
        .enumerate()
        .map(|(i, doc)| {
            let payload: Payload = json!({
                "id": doc.id,
                "text": doc.text,
            })
            .try_into()
            .unwrap();
            PointStruct::new(PointId::from(doc.id), embeddings[i].clone(), payload)
        })
        .collect();

    qdrant_client
        .upsert_points(UpsertPointsBuilder::new("documents", points))
        .await?;

    println!("Sucessfully created collection & inserted points");

    let query_text = "Tell me about rust programming language";
    let query_embedding = generate_embedding_for_text(query_text).await;

    let search_result = qdrant_client
        .search_points(SearchPointsBuilder::new("documents", query_embedding, 1).with_payload(true))
        .await?;

    println!("Search result: {:#?}", search_result);
    Ok(())
}

async fn generate_embeddings(texts: &[&str]) -> Vec<Vec<f32>> {
    let mut embeddings = Vec::with_capacity(texts.len());

    for text in texts {
        let embedding = generate_embedding_for_text(text).await;
        embeddings.push(embedding);
    }
    embeddings
}

async fn generate_embedding_for_text(text: &str) -> Vec<f32> {
    let client = Client::new();

    let response = client
        .post(OLLAMA_API_URL)
        .json(&serde_json::json!({
            "model": OLLAMA_MODEL,
            "input": text
        }))
        .send()
        .await
        .expect("Failed to generate embedding");

    dbg!(response.status());

    let embedding_data: OllamaEmbeddingResponse =
        response.json().await.expect("Failed to parse response");

    embedding_data
        .embeddings
        .first()
        .cloned()
        .expect("No embedding found")
}
