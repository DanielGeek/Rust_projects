use rust_bert::pipelines::{common::ModelType, translation::{Language, TranslationModelBuilder}};
use tch::Device;



fn main() {
  translation();
}

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