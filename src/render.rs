use std::f64::consts::PI;
use piet_common::{self as piet, kurbo, FontFamily, RenderContext, Text, TextLayoutBuilder, TextLayout};
use png::{ColorType, Encoder};

use crate::chronicler_schema::GameUpdateData;

const WIDGET_WIDTH: f64 = 308.0;
const WIDGET_HEIGHT: f64 = 172.0;
const CONTENT_WIDTH: f64 = 260.0;
const PADDING_Y: f64 = 20.0;
const BASE_WIDTH: f64 = 70.31;
const BASE_SPACING: f64 = 80.7;
const BASES_HEIGHT: f64 = 77.25;
const BASES_WIDTH: f64 = 100.0;
const COUNT_X: f64 = 130.0;
const COUNT_LINE_SPACING: f64 = 24.0;
const TEXT_BASELINE: f64 = 8.0;
const COUNT_WIDTH: f64 = 130.0;
const COUNT_DOT_RADIUS: f64 = 6.625;
const COUNT_DOT_BASELINE: f64 = 15.0;
const COUNT_DOT_SPACING: f64 = 17.0;
const BOTTOM_ROW_Y: f64 = 87.25;
const BOTTOM_ROW_LINE_SPACING: f64 = 32.0;
const TEXT_BACKGROUND_HEIGHT: f64 = 24.0;
const TEXT_BACKGROUND_PADDING_X: f64 = 10.0;
const TEXT_BACKGROUND_PADDING_Y: f64 = 3.0;

pub fn render(game_update: GameUpdateData) -> Result<Vec<u8>, piet_common::Error> {
    let mut device = piet::Device::new()?;
    let mut target = device.bitmap_target(WIDGET_WIDTH.ceil() as usize, WIDGET_HEIGHT.ceil() as usize, 1.0)?;
    let mut context = target.render_context();

    let font_family = context.text()
        .font_family("Open Sans")
        .or_else(|| context.text().font_family("Helvetica Neue"))
        .unwrap_or(piet::FontFamily::SANS_SERIF);
    info!("Using font {:?}", font_family);


    render_background(&mut context);
    // Apply padding
    context.transform(kurbo::Affine::translate(((WIDGET_WIDTH - CONTENT_WIDTH) / 2., PADDING_Y)));
    render_bases(&mut context, &game_update)?;
    render_count(&mut context, &font_family, &game_update)?;
    render_bottom_row(&mut context, &font_family, &game_update)?;

    context.finish()?;
    encode(target)
}

fn encode(mut target: piet::BitmapTarget) -> Result<Vec<u8>, piet_common::Error> {
    let image = target.to_image_buf(piet::ImageFormat::RgbaPremul)?;

    let mut buf = Vec::new();
    let mut encoder = Encoder::new(&mut buf, image.size().width as u32, image.size().height as u32);
    encoder.set_color(ColorType::Rgba);
    let mut writer = encoder.write_header().map_err(Into::<Box<_>>::into)?;
    writer.write_image_data(image.raw_pixels()).map_err(Into::<Box<_>>::into)?;
    writer.finish().map_err(Into::<Box<_>>::into)?;

    Ok(buf)
}

fn render_background(context: &mut impl RenderContext) {
    let shape = kurbo::RoundedRect::new(0., 0., WIDGET_WIDTH as f64, WIDGET_HEIGHT as f64, 5.);
    let brush = piet::Color::rgb(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0);
    context.fill(shape, &brush);
}

fn render_bases(context: &mut impl RenderContext, game: &GameUpdateData) -> Result<(), piet_common::Error> {
    if game.num_bases() != 4 {
        warn!("Can't handle {} bases, rendering 4 instead", game.num_bases());
    }

    context.save()?;
    let base_rect = kurbo::Rect::new(0., 0., BASE_WIDTH, BASE_WIDTH);
    let width = (BASE_WIDTH + BASE_SPACING) * 2.0_f64.sqrt();
    let scale_factor = BASES_WIDTH / width;
    context.transform(kurbo::Affine::translate((BASES_WIDTH, BASES_HEIGHT)));
    context.transform(kurbo::Affine::scale(scale_factor));
    context.transform(kurbo::Affine::translate((0., -((BASES_HEIGHT + BASE_SPACING) / 2.) / 2.0_f64.sqrt())));
    context.transform(kurbo::Affine::rotate(3.0 * PI / 4.0));
    for i in 0..3 {
        if game.bases_occupied.contains(&i) {
            context.fill(base_rect, &piet::Color::WHITE);
        } else {
            context.stroke(base_rect, &piet::Color::WHITE, 4.0);
        }
        if i % 2 == 0 {
            context.transform(kurbo::Affine::translate((0.0, BASE_SPACING)))
        } else {
            context.transform(kurbo::Affine::translate((BASE_SPACING, 0.0)))
        }
    }

    context.restore()
}

fn render_count(context: &mut impl RenderContext, font_family: &piet::FontFamily, game: &GameUpdateData) -> Result<(), piet_common::Error> {
    context.save()?;
    context.transform(kurbo::Affine::translate((COUNT_X, 0.)));

    render_count_line(context, font_family, "BALLS", game.max_balls(), game.at_bat_balls)?;
    context.transform(kurbo::Affine::translate((0., COUNT_LINE_SPACING)));
    render_count_line(context, font_family, "STRIKES", game.max_strikes(), game.at_bat_strikes)?;
    context.transform(kurbo::Affine::translate((0., COUNT_LINE_SPACING)));
    render_count_line(context, font_family, "OUTS", game.max_outs(), game.half_inning_outs)?;

    context.restore()
}

fn render_count_line(context: &mut impl RenderContext, font_family: &FontFamily, content: &'static str, n_dots: i32, n_filled: i32) -> Result<(), piet_common::Error> {
    context.save()?;

    render_text(context, font_family, content)?;

    let dot_start = COUNT_WIDTH - COUNT_DOT_SPACING * (n_dots - 1) as f64 + COUNT_DOT_RADIUS;
    context.transform(kurbo::Affine::translate((dot_start, 0.)));
    let circle = kurbo::Circle::new((0., COUNT_DOT_BASELINE), COUNT_DOT_RADIUS);
    for dot_i in 0..(n_dots - 1) {
        if dot_i < n_filled {
            context.fill(circle, &piet::Color::WHITE);
        } else {
            context.stroke(circle, &piet::Color::WHITE, 1.25);
        }
        context.transform(kurbo::Affine::translate((COUNT_DOT_SPACING, 0.)));
    }

    context.restore()
}

fn render_text(context: &mut impl RenderContext, font_family: &FontFamily, content: &'static str) -> Result<(), piet_common::Error> {
    let text = context.text().new_text_layout(content)
        .font(font_family.clone(), 14.)
        .text_color(piet::Color::WHITE)
        .default_attribute(piet::FontWeight::new(700))
        .build()?;

    context.draw_text(&text, (0., TEXT_BASELINE));

    Ok(())
}

fn render_bottom_row(context: &mut impl RenderContext, font_family: &piet::FontFamily, game: &GameUpdateData) -> Result<(), piet_common::Error> {
    context.save()?;
    context.transform(kurbo::Affine::translate((0., BOTTOM_ROW_Y)));

    render_text(context, font_family, "PITCHING")?;
    if let Some(color) = game.pitching_team_color() {
        render_name(context, font_family, game.current_pitcher_name(), color)?;
    }

    context.transform(kurbo::Affine::translate((0., BOTTOM_ROW_LINE_SPACING)));

    render_text(context, font_family, "BATTING")?;
    if let Some(color) = &game.batting_team_color() {
        render_name(context, font_family, game.current_batter_name(), color)?;
    }

    context.restore()
}

fn render_name(context: &mut impl RenderContext, font_family: &FontFamily, name: &Option<String>, color: &str) -> Result<(), piet_common::Error> {
    let mut display_name = name.as_ref().cloned().unwrap_or("-".to_string());
    if display_name.is_empty() { display_name = "-".to_string() }

    let text = context.text().new_text_layout(display_name)
        .font(font_family.clone(), 14.)
        .text_color(piet::Color::WHITE)
        .build()?;
    let size = text.size();

    let rect = kurbo::Rect::new(CONTENT_WIDTH - size.width - TEXT_BACKGROUND_PADDING_X,
                                TEXT_BASELINE - TEXT_BACKGROUND_PADDING_Y,
                                CONTENT_WIDTH + TEXT_BACKGROUND_PADDING_X,
                                TEXT_BACKGROUND_HEIGHT + TEXT_BACKGROUND_PADDING_Y)
        .to_rounded_rect(5.);
    context.fill(rect, &piet::Color::from_hex_str(color).map_err(Into::<Box<_>>::into)?.with_alpha(0.5));
    context.draw_text(&text, (CONTENT_WIDTH - size.width, TEXT_BASELINE));

    Ok(())
}