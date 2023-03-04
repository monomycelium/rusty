use lta::{LTAClient, Client, Bus, BusRequests, models::bus::bus_arrival};
use chrono::{NaiveDateTime, Local, Duration};
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
pub struct Input {
    #[arg(long)]
    /// api key for lta
    #[arg(env = "API_KEY")]
    api_key: String,
    /// number of timings
    #[arg(short, long)]
    #[arg(env = "TIMINGS")]
    #[arg(default_value = "3")]
    timings: u8,
    /// bus stop code
    #[arg(env = "BUS_STOP_CODE")]
    // #[arg(default_value = "43389")]
    bus_stop_code: u32,
    /// service number
    #[arg(env = "SERVICE_NO")]
    // #[arg(default_value = "189")]
    service_no: String,
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    next_bus(input.api_key, input.timings, input.bus_stop_code, input.service_no.as_str())
}

#[tokio::main]
pub async fn next_bus<'a>(api_key: String, timings: u8, bus_stop_code: u32, service_no: &'a str) -> Result<(), Box<dyn Error>> {
    let client: LTAClient = LTAClient::with_api_key(api_key)?;
    let bus_arrival: bus_arrival::BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, service_no).await?;

    if let Some(thing) = bus_arrival.services.first() {
        let now: NaiveDateTime = Local::now().naive_local();
        let mut count: u8 = 0;

        for next in thing.next_bus.iter().flatten().map(|next_bus: &bus_arrival::NextBus| next_bus.est_arrival.naive_local()).filter(|time: &NaiveDateTime| time > &now) {
            let diff: Duration = next - now;
            println!("{:02}:{:02}", diff.num_minutes(), diff.num_seconds() % 60);
            count += 1;

            if count == timings {
                break;
            }
        }

        if count == 0 {
            Err("no estimated arrivals found; try again later".into())
        } else {
            Ok(())
        }
    } else {
        Err("no data found; try a different bus stop code or service number".into())
    }
}

// #[tokio::main]
// pub async fn next_bus_detailed<'a>(api_key: String, bus_stop_code: u32, service_no: &'a str) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

// #[tokio::main]
// pub async fn next_bus_detailed_all<'a>(api_key: String, bus_stop_code: u32) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }
