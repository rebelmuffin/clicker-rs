use macroquad::prelude::*;

use crate::game_types::*;
use crate::ui::*;

const CIRCLE_INTERP_SPEED: f32 = 10.0;
const CIRCLE_COLOR: Color = RED;

fn interp_colours(a: Color, b: Color, alpha: f32) -> Color {
    return Color::new(
        (b.r - a.r) * alpha,
        (b.g - a.g) * alpha,
        (b.b - a.b) * alpha,
        (b.a - a.a) * alpha,
    );
}

pub struct GameUI<'a> {
    game: &'a mut GameState,
    initial_click: bool,

    current_circle_radius: f32,
    current_circle_colour: Color,
}

impl<'a> GameUI<'a> {
    pub fn new(game: &mut GameState) -> GameUI {
        return GameUI {
            current_circle_colour: CIRCLE_COLOR,
            current_circle_radius: game.calculate_circle_radius(),
            game,
            initial_click: false,
        };
    }

    fn draw_upgrades(&mut self) {
        let mut upgrade_buttons: Vec<Button> = Vec::new();
        let mut upgrade_idx = 0;
        let mut max_width = 0.0;
        while upgrade_idx < self.game.upgrades.len() {
            let button = self.create_upgrade_button(upgrade_idx);

            if button.rect.w > max_width {
                max_width = button.rect.w
            }

            upgrade_buttons.push(button);
            upgrade_idx += 1;
        }

        for (i, button) in upgrade_buttons.iter_mut().enumerate() {
            button.rect.w = max_width;

            if button.is_pressed() {
                self.game.buy_upgrade(i);
            }

            button.draw();
        }
    }

    fn draw_game_circle(&mut self, delta_ms: i64) {
        let circle = Circle::new(
            screen_width() / 2.0,
            screen_height() / 2.0,
            self.current_circle_radius,
        );
        let mut target_circle_colour = CIRCLE_COLOR;
        let mut target_circle_radius = self.game.calculate_circle_radius();

        let released = is_mouse_button_released(MouseButton::Left);
        let down = is_mouse_button_down(MouseButton::Left);

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_circ = Circle::new(mouse_x, mouse_y, 1.0);
        if mouse_circ.overlaps(&circle) {
            if released {
                self.game.money += self.game.click_value;
                if self.initial_click == false {
                    self.initial_click = true;
                }
            }
            if down {
                target_circle_radius *= 0.9
            }
            target_circle_colour = Color::new(
                target_circle_colour.r * 0.8,
                target_circle_colour.g * 0.8,
                target_circle_colour.b * 0.8,
                target_circle_colour.a,
            );
        }

        let interp_alpha = 0.9 * (delta_ms as f32 / 1000.0) * CIRCLE_INTERP_SPEED;
        // circle target radius interpolation
        self.current_circle_radius +=
            (target_circle_radius - self.current_circle_radius) * interp_alpha;

        // circle target colour interpolation
        self.current_circle_colour.add(interp_colours(
            self.current_circle_colour,
            target_circle_colour,
            interp_alpha,
        ));

        const OUTLINE_WIDTH: f32 = 2.0;
        draw_circle(
            circle.x,
            circle.y,
            self.current_circle_radius + OUTLINE_WIDTH,
            Color::from_hex(0x552211),
        );
        draw_circle(
            circle.x,
            circle.y,
            self.current_circle_radius,
            self.current_circle_colour,
        );
    }

    fn draw_stats(&self) {
        draw_text(
            format!("Money: {}", self.game.money).as_str(),
            25.0,
            25.0,
            24.0,
            RED,
        );
        draw_text(
            format!("Click Value: {}", self.game.click_value).as_str(),
            25.0,
            25.0 + 24.0,
            24.0,
            RED,
        );
    }

    fn draw_click_prompt(&self) {
        let prompt_text = "Click on the ball!";
        let prompt_font_size = 48.0;
        let text_width = measure_text(prompt_text, None, prompt_font_size as u16, 1.0).width;

        let prompt_x = screen_width() / 2.0 - text_width / 2.0;
        let prompt_y = screen_height() / 4.0;
        draw_text(prompt_text, prompt_x, prompt_y, prompt_font_size, BLUE);
    }

    pub fn draw_game(&mut self, delta_ms: i64) {
        self.draw_game_circle(delta_ms);
        self.draw_upgrades();
        self.draw_stats();

        if self.initial_click == false {
            self.draw_click_prompt();
        }
    }
}

impl<'a> GameUI<'a> {
    fn create_upgrade_button(&self, idx: usize) -> Button {
        let text: String = if self.game.upgrades[idx].is_active {
            String::from("ACTIVE")
        } else {
            format!("COST: {}", self.game.upgrades[idx].cost).to_string()
        };

        let colour = if self.game.upgrades[idx].is_active {
            GREEN
        } else {
            if self.game.can_afford(idx) {
                BLUE
            } else {
                RED
            }
        };
        let disabled = self.game.upgrades[idx].is_active;

        const WIDTH: f32 = 150.0;
        const HEIGHT: f32 = 50.0;
        const PADDING: f32 = 10.0;
        const X_OFFSET: f32 = 25.0;
        const Y_OFFSET: f32 = 60.0;
        let x = X_OFFSET;
        let y = (HEIGHT + PADDING) * idx as f32 + Y_OFFSET;
        return Button::new(Rect::new(x, y, WIDTH, HEIGHT), colour, text, disabled, true);
    }
}
