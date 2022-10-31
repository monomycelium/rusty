use lta::{LTAResult, LTAClient, Client, Bus, BusRequests};

fn get_bus_arrival() -> LTAResult<()> {
    let api_key = std::env::var("API_KEY").expect("API_KEY not found!");
    let client = LTAClient::with_api_key(api_key);
    let arrivals = Bus::get_arrival(&client, 83139, None)?;
    println!("{:?}", arrivals);
    Ok(())
}
