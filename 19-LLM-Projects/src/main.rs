use kalosm::{language::*, sound::{AsyncSourceTranscribeExt, MicInput, Whisper}, vision::{Ocr, OcrInferenceSettings, Wuerstchen, WuerstchenInferenceSettings}};
use image::{self, GenericImageView};

#[tokio::main]
async fn main() {
    // process_text().await;
    // ocr().await;
    // audio_transcription().await;
    image_generation().await;
}

pub async fn image_generation() {
    let model = Wuerstchen::builder()
        .build()
        .await
        .unwrap();

    let settings = WuerstchenInferenceSettings::new(
        "A cat "
    );

    if let Ok(mut images) = model.run(settings) {
        while let Some(image) = images.next().await {
            if let Some(buf) = image.generated_image() {
                buf.save(&format!("{}.png", image.sample_num())).unwrap();
            }
        }
    }
}

pub async fn audio_transcription() {
    let model = Whisper::new()
        .await
        .unwrap();

    let mic = MicInput::default();
    let stream = mic.stream().unwrap();

    let mut text_stream = stream.transcribe(model);

    text_stream.to_std_out().await.unwrap();
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
