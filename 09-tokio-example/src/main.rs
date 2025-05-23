use tokio::time::{sleep, Duration};
use log::Level;

async fn sleeper(name: &str) {
  log::info!("{}: Sleeping", name);
  sleep(Duration::from_secs(1)).await;
  log::info!("{}: Awake!", name);
}

// async fn reader() {
//   log::info!("Reading some beeg data");
//   let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
//   let mut contents = String::new();
//   f.read_to_string(&mut contents).await.unwrap();
//   log::info!("Read beeg {} bytes", contents.len());
// }

async fn run() {
  sleeper("First").await;
  sleeper("Second").await;
  // tokio::join!(
  //   sleeper(),
  //   reader(),
  // );
}

#[tokio::main]
async fn main() {
  simple_logger::init_with_level(Level::Info).unwrap();

  let start = std::time::Instant::now();
  run().await;
  let end = std::time::Instant::now();
  
  println!("Took {:?} seconds", end - start);
  // let rt = tokio::runtime::Runtime::new().unwrap();
  // let future = run();

  // rt.block_on(future);
}
