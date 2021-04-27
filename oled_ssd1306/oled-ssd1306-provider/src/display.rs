use anyhow::{anyhow, Result};
use embedded_graphics::{fonts::Font12x16, pixelcolor::BinaryColor, prelude::*};
use embedded_text::prelude::*;
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, Builder, I2CDIBuilder};

pub fn say(text: &str) -> Result<()> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDIBuilder::new().init(i2c);
    let mut display: GraphicsMode<_, _> = Builder::new().connect(interface).into();

    display
        .init()
        .map_err(|_| anyhow!("error initializing display"))?;

    let textbox_style = TextBoxStyleBuilder::new(Font12x16)
        .text_color(BinaryColor::On)
        .height_mode(FitToText)
        .build();

    let bounds = Rectangle::new(Point::zero(), Point::new(128, 0));

    TextBox::new(text, bounds)
        .into_styled(textbox_style)
        .draw(&mut display)
        .map_err(|e| anyhow!("error writing text {:?}", e))?;

    display
        .flush()
        .map_err(|e| anyhow!("error flushing display {:?}", e))
}
