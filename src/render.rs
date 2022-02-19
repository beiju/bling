use std::io::BufWriter;
use piet_common::{Device, RenderContext, kurbo, Color, ImageFormat};
use png::{ColorType, Encoder};

use crate::chronicler_schema::GameUpdateData;

const OUTPUT_WIDTH: usize = 308;
const OUTPUT_HEIGHT: usize = 308;

pub fn render(game_update: GameUpdateData) -> Result<Vec<u8>, piet_common::Error> {
    let mut device = Device::new()?;
    let mut target = device.bitmap_target(308, 192, 1.0)?;
    let mut context = target.render_context();

    render_background(&mut context);

    context.finish()?;
    let image = target.to_image_buf(ImageFormat::RgbaPremul)?;

    let mut buf = Vec::new();
    let mut encoder = Encoder::new(&mut buf, image.size().width as u32, image.size().height as u32);
    encoder.set_color(ColorType::Rgba);
    let mut writer = encoder.write_header().map_err(Into::<Box<_>>::into)?;
    writer.write_image_data(image.raw_pixels()).map_err(Into::<Box<_>>::into)?;
    writer.finish().map_err(Into::<Box<_>>::into)?;

    Ok(buf)
}

fn render_background(context: &mut impl RenderContext) {
    let shape = kurbo::RoundedRect::new(0., 0., OUTPUT_WIDTH as f64, OUTPUT_HEIGHT as f64, 5.);
    let brush = Color::rgb(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0);
    context.fill(shape, &brush);
}