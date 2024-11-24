use rust_bert::pipelines::{common::ModelType, pos_tagging::POSModel, sentiment::SentimentModel, translation::{Language, TranslationModelBuilder}};
use tch::Device;



fn main() {
  sentimental_analysis();
  // pos();
  // translation();
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