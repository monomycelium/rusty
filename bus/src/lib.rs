use lta::{LTAClient, Client, Bus, BusRequests, models::bus::bus_arrival::BusArrivalResp};
use chrono::{NaiveTime, Local, Duration};
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
pub struct Input {}

pub fn run(input: Input) {
    
}

#[tokio::main]
async fn get_arrival(api_key: String, bus_stop_code: u32, service_no: &str) -> Result<(), Box<dyn Error>> {
    let client: LTAClient = LTAClient::with_api_key(api_key)?;
    let mut bus_arrival: BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, service_no).await?;

    let time: NaiveTime = bus_arrival.services[0].next_bus[0].as_mut().unwrap().est_arrival.time();

    // let time: NaiveTime = NaiveTime::parse_from_str("2023-03-03T13:18:33+08:00", "%Y-%m-%dT%H:%M:%S+08:00")?;
    let diff: Duration = time - Local::now().time();

    println!("{:02}:{:02}", diff.num_seconds() / 60, diff.num_seconds() % 60);

    Ok(())
}
