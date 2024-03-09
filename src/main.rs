mod game_setup;
mod game_types;
mod ui;
mod upgrades;

use chrono::prelude::*;
use macroquad::prelude::*;

use game_setup::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = create_default_game_state();
    let mut game_ui = ui::GameUI::new(&mut game_state);

    let mut last_update = Utc::now();
    loop {
        let now = Utc::now();
        let delta_time_us = match (now - last_update).num_microseconds() {
            Some(v) => v,
            None => 1,
        };
        last_update = now;

        // draw
        clear_background(Color::from_hex(0x0f0f1a));

        let before_draw = Utc::now();
        game_ui.draw_game(delta_time_us / 1000);
        let after_draw = Utc::now();

        // draw frametime
        let delta_time_text = &format!(
            "{} FPS | {}us",
            1000000 / if delta_time_us != 0 { delta_time_us } else { 1 },
            delta_time_us
        );
        draw_text(delta_time_text, screen_width() / 2.0, 25.0, 24.0, RED);

        // draw draw time
        let draw_time_us = match (after_draw - before_draw).num_microseconds() {
            Some(v) => v,
            None => 1,
        };
        let draw_time_text = &format!("Draw: {}us", draw_time_us);
        draw_text(draw_time_text, screen_width() / 2.0, 25.0 + 24.0, 24.0, RED);

        next_frame().await
    }
}
