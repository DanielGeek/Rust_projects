use kalosm::{language::*, vision::{Ocr, OcrInferenceSettings}};
use image::{self, GenericImageView};

#[tokio::main]
async fn main() {
    // process_text().await;
    ocr().await;
}

pub async fn ocr() {
    let mut model = Ocr::builder()
        .build()
        .await
        .unwrap();

    let image = image::open("../19-LLM-Projects/src/ocr.png")
        .unwrap().grayscale();

    println!("Image dimensions: {:?}", image.dimensions());

    let text = model
        .recognize_text(OcrInferenceSettings::new(image).unwrap())
        .unwrap();

    println!("Got from Image: {}", text);
}

pub async fn process_text() {
    let model = Llama::new_chat().await
        .unwrap();

    let mut chat = Chat::builder(model)
        .with_system_prompt("The assistan will act like a doctor")
        .build();

    loop {
        chat.add_message(prompt_input("\n> ").unwrap())
            .to_std_out()
            .await
            .unwrap();
    }
}
