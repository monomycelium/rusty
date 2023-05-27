use std::time::Duration;
use tm1637_gpio_driver::TM1637Adapter;

fn main() {
    let mut display: TM1637Adapter = tm1637_gpio_driver::gpio_api::setup_gpio_cdev(
        24,
        11,
        Box::from(|| std::thread::sleep(Duration::from_micros(10))),
        "/dev/gpiochip0",
    );

    if let Err(e) = segments::run(&mut display, env!("API_KEY"), 43389, "189") {
        eprintln!("error: {}.", e);
        display.write_segments_raw(&segments::encode_char_array(['E', 'r', 'r', ' ']), 0);
        std::process::exit(1);
    }
}
