use charlcd::Screen;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::default()?;

    screen.clear()?;
    screen.write(b"hello, world!")?;
    screen.flash_backlight()?;
    screen.flush()?; // send all the previous commands to the driver at once

    Ok(())
}
