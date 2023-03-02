use pwr_hd44780::Hd44780;

fn main() {
    let lcd_bus = pwr_hd44780::I2CBus::new("/dev/i2c-1", 0x3f).unwrap();
    let mut lcd = pwr_hd44780::DirectLcd::new(Box::new(lcd_bus),16, 2).unwrap();

    lcd.clear().unwrap();
    lcd.print("hello world").unwrap();
}
