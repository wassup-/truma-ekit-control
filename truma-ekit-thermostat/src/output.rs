use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_6X10},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{renderer::TextRenderer, Alignment, Baseline, Text, TextStyle},
    Drawable,
};
use esp_idf_hal::{
    gpio::AnyIOPin,
    i2c::{I2c, I2cConfig, I2cDriver},
    peripheral::Peripheral,
};
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::WriteOnlyDataCommand,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};
use truma_ekit_core::{measurement::Formatter as MeasurementFormatter, types::Temperature};

#[derive(Debug)]
pub struct Output {
    pub requested_temperature: Temperature,
    pub actual_temperature: Option<Temperature>,
    pub wifi_connected: bool,
}

pub fn display<'a, I2C: I2c>(
    i2c: impl Peripheral<P = I2C> + 'a,
    sda: AnyIOPin,
    scl: AnyIOPin,
) -> impl FnMut(Output) + 'a {
    let driver = I2cDriver::new(i2c, sda, scl, &I2cConfig::default()).unwrap();
    let interface = I2CDisplayInterface::new(driver);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let normal_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let large_text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    move |output| render_output(&mut display, output, normal_text_style, large_text_style)
}

fn render_output<DI, TN, TL>(
    display: &mut Ssd1306<DI, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    output: Output,
    normal_text_style: TN,
    large_text_style: TL,
) where
    DI: WriteOnlyDataCommand,
    TN: TextRenderer<Color = BinaryColor> + Clone,
    TL: TextRenderer<Color = BinaryColor>,
{
    println!("output: {:?}", output);

    display.clear();

    let formatter = MeasurementFormatter::with_precision(1);

    // actual temperature
    let actual_temperature = if let Some(actual_temperature) = output.actual_temperature {
        formatter.format(&actual_temperature)
    } else {
        String::from("??")
    };
    Text::with_alignment(
        &actual_temperature,
        Point::new(64, 32),
        large_text_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap_or_else(|_| panic!("failed to render output"));

    // requested temperature
    let requested_temperature = formatter.format(&output.requested_temperature);
    let requested_temperature = format!("Requested: {}", requested_temperature);
    Text::with_alignment(
        &requested_temperature,
        Point::new(64, 48),
        normal_text_style.clone(),
        Alignment::Center,
    )
    .draw(display)
    .unwrap_or_else(|_| panic!("failed to render output"));

    // wifi status
    let status = if output.wifi_connected {
        "CONNECTED"
    } else {
        "DISCONNECTED"
    };
    let mut text_style = TextStyle::with_alignment(Alignment::Right);
    text_style.baseline = Baseline::Top;

    Text::with_text_style(status, Point::new(128, 0), normal_text_style, text_style)
        .draw(display)
        .unwrap_or_else(|_| panic!("failed to render output"));

    display.flush().unwrap();
}
