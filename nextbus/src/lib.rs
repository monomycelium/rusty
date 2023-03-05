use lta::{LTAClient, Client, Bus, BusRequests, models::bus::bus_arrival, models::bus_enums};
use chrono::{NaiveDateTime, Local, Duration};
use clap::Parser;
use colored::{Colorize, ColoredString};
use std::error::Error;

#[derive(Parser)]
#[command(about = "a sly command-line tool to show bus arrivals.")]
pub struct Input {
    #[arg(long)]
    /// api key for lta
    #[arg(env = "API_KEY")]
    api_key: String,
    /// number of estimated arrival times
    #[arg(short, long)]
    // #[arg(env = "TIMES")]
    #[arg(default_value = "3")]
    times: u8,
    /// express detailed information like bus load
    #[arg(short, long)]
    // #[arg(env = "DETAILED")]
    #[arg(default_value = "false")]
    detailed: bool,
    /// bus stop code
    // #[arg(env = "BUS_STOP_CODE")]
    // #[arg(default_value = "43389")]
    bus_stop_code: u32,
    /// service number
    // #[arg(env = "SERVICE_NO")]
    // #[arg(default_value = "189")]
    service_no: Option<String>,
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let client: LTAClient = LTAClient::with_api_key(input.api_key)?;

    if let Some(service_no) = input.service_no {
        if input.detailed {
            next_bus_detailed(&client, input.times, input.bus_stop_code, service_no.as_str())?;
        } else {
            next_bus(&client, input.times, input.bus_stop_code, service_no.as_str())?;
        }
    } else {
        if input.detailed {
            next_busses_detailed(&client, input.bus_stop_code)?;
        } else {
            next_busses(&client, input.bus_stop_code)?;
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn next_bus<'a>(client: &LTAClient, times: u8, bus_stop_code: u32, service_no: &'a str) -> Result<(), Box<dyn Error>> {
    let bus_arrival: bus_arrival::BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, service_no).await?;

    if let Some(service) = bus_arrival.services.first() {
        let now: NaiveDateTime = Local::now().naive_local();
        let mut count: u8 = 0;

        println!("{}:", service.service_no);

        for next in service.next_bus.iter().flatten().map(|next_bus: &bus_arrival::NextBus| next_bus.est_arrival.naive_local()) {
            if now > next  {
                println!("arriving")
            } else {
                let diff: Duration = next - now;
                println!("{:02}:{:02}", diff.num_minutes(), diff.num_seconds() % 60);
            }
            
            count += 1;

            if count == times {
                break;
            }
        }

        if count == 0 {
            Err("no estimated arrival times found; try again later".into())
        } else {
            Ok(())
        }
    } else {
        Err("bus not in operation; try entering a different bus stop or service".into())
    }
}

#[tokio::main]
pub async fn next_bus_detailed<'a>(client: &LTAClient, times: u8, bus_stop_code: u32, service_no: &'a str) -> Result<(), Box<dyn Error>> {
    let bus_arrival: bus_arrival::BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, service_no).await?;

    if let Some(service) = bus_arrival.services.first() {
        let mut count: u8 = 0;

        println!("{}:", service.service_no);

        for next in service.next_bus.iter().flatten() {
            println!("{}  {} {}", load_match(diff(next.est_arrival.naive_local()), &next.load), if next.feature == Some(bus_enums::BusFeature::WheelChairAccessible) { " ♿" } else { "" }, type_match(&next.bus_type));
            
            count += 1;
            if count == times {
                break;
            }
        }

        if count == 0 {
            Err("no estimated arrival times found; try again later".into())
        } else {
            Ok(())
        }
    } else {
        Err("bus not in operation; try entering a different bus stop or service".into())
    }
}

#[tokio::main]
pub async fn next_busses<'a>(client: &LTAClient, bus_stop_code: u32) -> Result<(), Box<dyn Error>> {
    let bus_arrival: bus_arrival::BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, None).await?;
    let now: NaiveDateTime = Local::now().naive_local();

    if bus_arrival.services.len() == 0 {
        Err("no bus in operation; try entering a different bus stop".into())
    } else {        
        for service in bus_arrival.services {
            if service.next_bus.iter().flatten().count() > 0 {
                println!("{}:", service.service_no);
                
                for next in service.next_bus.iter().flatten().map(|next_bus: &bus_arrival::NextBus| next_bus.est_arrival.naive_local()) {
                    if now > next  {
                        println!("arriving")
                    } else {
                        let diff: Duration = next - now;
                        println!("{:02}:{:02}", diff.num_minutes(), diff.num_seconds() % 60);
                    }
                }

                println!();
            }
        }

        Ok(())
    }
}

#[tokio::main]
pub async fn next_busses_detailed<'a>(client: &LTAClient, bus_stop_code: u32) -> Result<(), Box<dyn Error>> {
    let bus_arrival: bus_arrival::BusArrivalResp = Bus::get_arrival(&client, bus_stop_code, None).await?;

    if bus_arrival.services.len() == 0 {
        Err("no bus in operation; try entering a different bus stop".into())
    } else {        
        for service in bus_arrival.services {
            if service.next_bus.iter().flatten().count() > 0 {
                println!("{}:", service.service_no);
                
                for next in service.next_bus.iter().flatten() {
                    println!("{}  {} {}", load_match(diff(next.est_arrival.naive_local()), &next.load), if next.feature == Some(bus_enums::BusFeature::WheelChairAccessible) { " ♿" } else { "" }, type_match(&next.bus_type));
                }

                println!();
            }
        }

        Ok(())
    }
}

fn diff(est: NaiveDateTime) -> String {
    let now: NaiveDateTime = Local::now().naive_local();

    if now > est {
        format!("arriving")
    } else {
        let diff: Duration = est - now;
        format!("{:02}:{:02}   ", diff.num_minutes(), diff.num_seconds() % 60)
    }
}

fn load_match(diff: String, load: &bus_enums::BusLoad) -> ColoredString {
    match load {
        bus_enums::BusLoad::LimitedStanding => diff.red(),
        bus_enums::BusLoad::StandingAvailable => diff.yellow(),
        bus_enums::BusLoad::SeatsAvailable => diff.green(),
        bus_enums::BusLoad::Unknown => diff.normal(),
    }
}

fn type_match(bus_type: &bus_enums::BusType) -> ColoredString {
    match bus_type {
        bus_enums::BusType::SingleDecker => "single-decker",
        bus_enums::BusType::DoubleDecker => "double-decker",
        bus_enums::BusType::Bendy => "bendy",
        bus_enums::BusType::Unknown => "unknown",
    }.dimmed()
}

/*
 * TODO:
 * - implement feature to display __when__ the next bus will arrive
 * - implement feature to refresh output continously
 */
