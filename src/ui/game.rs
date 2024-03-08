use macroquad::prelude::*;

use crate::game_types::*;
use crate::ui::*;

fn create_upgrade_button(game: &GameState, idx: usize) -> Button {
    let text: String = if game.upgrades[idx].is_active {
        String::from("ACTIVE")
    } else {
        format!("COST: {}", game.upgrades[idx].cost).to_string()
    };

    let colour = if game.upgrades[idx].is_active {
        GREEN
    } else {
        if game.can_afford(idx) {
            BLUE
        } else {
            RED
        }
    };
    let disabled = game.upgrades[idx].is_active;

    const WIDTH: f32 = 150.0;
    const HEIGHT: f32 = 50.0;
    const PADDING: f32 = 10.0;
    const X_OFFSET: f32 = 25.0;
    const Y_OFFSET: f32 = 60.0;
    let x = X_OFFSET;
    let y = (HEIGHT + PADDING) * idx as f32 + Y_OFFSET;
    return Button::new(Rect::new(x, y, WIDTH, HEIGHT), colour, text, disabled);
}

fn draw_upgrades(game: &mut GameState) {
    let mut upgrade_buttons: Vec<Button> = Vec::new();
    let mut upgrade_idx = 0;
    while upgrade_idx < game.upgrades.len() {
        let button = create_upgrade_button(&game, upgrade_idx);

        if button.is_pressed() {
            game.buy_upgrade(upgrade_idx);
        }

        upgrade_buttons.push(button);
        upgrade_idx += 1;
    }

    for button in upgrade_buttons {
        button.draw();
    }
}

fn draw_game_circle(game: &mut GameState) {
    let circle = Circle::new(
        screen_width() / 2.0,
        screen_height() / 2.0,
        game.calculate_circle_radius(),
    );
    let mut circle_colour = RED;

    let pressed = is_mouse_button_pressed(MouseButton::Left);
    let down = is_mouse_button_down(MouseButton::Left);

    if pressed || down {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_circ = Circle::new(mouse_x, mouse_y, 1.0);
        if mouse_circ.overlaps(&circle) {
            if pressed {
                game.money += game.click_value;
            }
            if down {
                circle_colour = BLUE;
            }
        }
    }

    draw_circle(circle.x, circle.y, circle.radius(), circle_colour);
}

fn draw_stats(game: &GameState) {
    draw_text(
        format!("Money: {}", game.money).as_str(),
        25.0,
        25.0,
        24.0,
        RED,
    );
    draw_text(
        format!("Click Value: {}", game.click_value).as_str(),
        25.0,
        25.0 + 24.0,
        24.0,
        RED,
    );
}

pub fn draw_game(game: &mut GameState) {
    draw_game_circle(game);
    draw_upgrades(game);
    draw_stats(game);
}
