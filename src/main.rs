use tokio;
use weather::Weather;

mod weather;
mod location;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather = Weather::get().await;
    print!("{}", weather);

    Ok(())
}
