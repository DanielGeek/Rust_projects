use std::fmt::format;

use rust_bert::pipelines::{common::ModelType, keywords_extraction::{KeywordExtractionConfig, KeywordExtractionModel, KeywordScorerType}, pos_tagging::POSModel, sentence_embeddings::{SentenceEmbeddingsConfig, SentenceEmbeddingsModelType}, sentiment::SentimentModel, translation::{Language, TranslationModelBuilder}, zero_shot_classification::ZeroShotClassificationModel};
use tch::Device;



fn main() {
  text_classification();
  // keyword_extraction();
  // sentimental_analysis();
  // pos();
  // translation();
}

// Text Classification function
fn text_classification() {
  let classification_model =
    ZeroShotClassificationModel::new(Default::default()).unwrap();

  let input1 = "Which sports do you play?";
  let input2 = "Who is the prime minister of your country?";

  let candidate_labels = &["politics", "sports", "public health", "drama", "universe"];

  let output = classification_model
    .predict_multilabel(
      [input1, input2],
      candidate_labels,
      Some(Box::new(|label: &str| {
        format!("Example is about {label}")
      })),
      128
    ).unwrap();
  
  println!("{output:?}");
}

// Keyword Extraction function
fn keyword_extraction() {
  let keyword_extraction_config = KeywordExtractionConfig {
    sentence_embeddings_config: SentenceEmbeddingsConfig::from(
      SentenceEmbeddingsModelType::AllMiniLmL6V2
    ),
    scorer_type: KeywordScorerType::MaxSum,
    ngram_range: (1, 1),
    num_keywords: 5,
    ..Default::default()
  };

  let keyword_extraction_model =
    KeywordExtractionModel::new(keyword_extraction_config).unwrap();

  let input = "The universe is all of space and time and their contents. It comprises all of existence, any fundamental interaction, physical process and physical constant, and therefore all forms of matter and energy, and the structures they form, from sub-atomic particles to entire galactic filaments. Since the early 20th century, the field of cosmology establishes that space and time emerged together at the Big Bang 13.787±0.020 billion years ago and that the universe has been expanding since then. The portion of the universe that we can see is approximately 93 billion light-years in diameter at present, but the total size of the universe is not known";

  let keywords = keyword_extraction_model.predict(&[input]).unwrap();

  for keyword in keywords[0].iter() {
    println!("{:?}, {:?}", keyword.text, keyword.score)
  }
}

// Sentimental Analysis function
fn sentimental_analysis() {
  let sentimental_classifier =
      SentimentModel::new(Default::default()).unwrap();

  let input = [
    "This movie is the worst i have seen",
    "This is the best youtube channel for rust programming language"
  ];

  let output = sentimental_classifier.predict(input);

  for sentiment in output {
    println!("{sentiment:?}");
  }
}

// Parts of Speech model function
fn pos() {
  let pos_model = POSModel::new(Default::default())
      .unwrap();

  let input = ["Hello, How are you doing?"];

  let output = pos_model.predict(&input);

  for (pos, tag) in output[0].iter().enumerate() {
    println!("{pos} - {tag:?}");
  }
}

// Translation model function
fn translation() {
  let model = TranslationModelBuilder::new()
      .with_device(Device::cuda_if_available())
      .with_model_type(ModelType::Marian)
      .with_source_languages(vec![Language::English])
      .with_target_languages(vec![Language::Spanish])
      .create_model()
      .unwrap();

  let text = "Hello, How are you? Please Subscribe!";

  let output = model
      .translate(
        &[text],
        None,
        Language::Spanish
      )
      .unwrap();

    for sentence in output {
      println!("{sentence}");
    }
}