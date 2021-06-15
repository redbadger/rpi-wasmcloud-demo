use anyhow::{anyhow, Result};
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub fn say(text: &str) -> Result<()> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .map_err(|_| anyhow!("error initializing display"))?;

    let character_style = MonoTextStyle::new(&FONT_9X18, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Center)
        .paragraph_spacing(6)
        .build();

    let bounds = Rectangle::new(Point::zero(), Size::new(128, 0));

    let text_box = TextBox::with_textbox_style(text, bounds, character_style, textbox_style);

    text_box
        .draw(&mut display)
        .map_err(|e| anyhow!("error writing to display {:?}", e))?;

    display
        .flush()
        .map_err(|e| anyhow!("error flushing display {:?}", e))
}
