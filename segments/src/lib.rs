use lta::{Bus, BusRequests, Client, LTAClient, LTAError};
use std::{fmt, thread, time::Duration};
use tm1637_gpio_driver::{
    fourdigit7segdis::DISPLAY_COUNT, mappings::SegmentBits, Brightness, DisplayState, TM1637Adapter,
};

pub const SECOND: Duration = Duration::from_secs(1);

pub enum SegmentsError {
    LTAError(LTAError),
    NotInOperation,
    NoEstAvailable,
}

impl From<LTAError> for SegmentsError {
    fn from(err: LTAError) -> Self {
        return Self::LTAError(err);
    }
}

impl std::fmt::Display for SegmentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LTAError(err) => return write!(f, "{}", err),
            Self::NotInOperation => return write!(f, "bus not in operation"),
            Self::NoEstAvailable => return write!(f, "no estimate available"),
        }
    }
}

#[tokio::main]
pub async fn fetch<'a, S>(
    client: &LTAClient,
    bus_stop_code: u32,
    service_no: S,
) -> Result<Vec<chrono::NaiveDateTime>, SegmentsError>
where
    S: Into<Option<&'a str>> + Send,
{
    Ok(Bus::get_arrival(&client, bus_stop_code, service_no)
        .await?
        .services
        .first()
        .ok_or(SegmentsError::NotInOperation)?
        .next_bus
        .iter()
        .flatten()
        .map(|arrival| arrival.est_arrival.naive_local())
        .collect())
}

pub fn display_loop(
    adapter: &mut TM1637Adapter,
    tick_fn: &dyn Fn(),
    time_fn: &dyn Fn(&mut u8) -> Option<[char; DISPLAY_COUNT]>,
) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dots: bool = false;
    let mut counter: u8 = 0;
    loop {
        let string: Option<[char; DISPLAY_COUNT]> = (time_fn)(&mut counter);
        if string.is_none() {
            return;
        };

        let mut data: [u8; DISPLAY_COUNT] = encode_char_array(string.unwrap());
        if show_dots {
            data[1] |= SegmentBits::SegPoint as u8;
        }

        adapter.write_segments_raw(&data, 0);
        (tick_fn)();
        show_dots = !show_dots;
    }
}

pub fn show(display: &mut TM1637Adapter, next: chrono::NaiveDateTime) {
    let callback_tick = || thread::sleep(SECOND);
    let callback_time = |_: &mut u8| -> Option<[char; DISPLAY_COUNT]> {
        let now: chrono::NaiveDateTime = chrono::Local::now().naive_local();

        if now < next {
            let diff: chrono::Duration = next - now;
            let (m, s) = (diff.num_minutes() as u8, diff.num_seconds() as u8 % 60);
            let digits: [u8; DISPLAY_COUNT] = [m / 10 % 10, m % 10, s / 10 % 10, s % 10];
            return Some(digits.map(|x| (x + b'0') as char));
        } else {
            return None;
        }
    };

    display_loop(display, &callback_tick, &callback_time);
}

pub fn encode_char_array(str: [char; DISPLAY_COUNT]) -> [u8; DISPLAY_COUNT] {
    let mut data: [u8; DISPLAY_COUNT] = [0; DISPLAY_COUNT];
    data.iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x = TM1637Adapter::encode_char(str[i]));

    return data;
}

pub fn run<'a, S, Str>(
    display: &mut TM1637Adapter,
    api_key: Str,
    bus_stop_code: u32,
    service_no: S,
) -> Result<(), SegmentsError>
where
    S: Into<Option<&'a str>> + Send,
    Str: Into<String>,
{
    let client: LTAClient = Client::with_api_key(api_key)?;

    let data: Vec<chrono::NaiveDateTime> = fetch(&client, bus_stop_code, service_no)?;

    data.iter().for_each(|x| {
        show(display, *x);
        display.write_segments_raw(&encode_char_array(['A', 'r', 'r', ' ']), 0);
        thread::sleep(SECOND * 5);
    });

    return Ok(());
}
