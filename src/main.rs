extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};

mod boids;
use crate::boids::Boids;

fn main() -> amethyst::Result<()> {
    use amethyst::utils::application_root_dir;
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;

    let mut app = Application::new("./", Boids, game_data)?;
    app.run();

    Ok(())
}
