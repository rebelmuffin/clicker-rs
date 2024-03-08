use macroquad::prelude::*;

#[allow(dead_code)] // is this even good practice
#[derive(Copy, Clone)]
pub enum ColourOption {
    Default(),
    Override(Color),
}

impl ColourOption {
    pub fn get_colour(self, default: Color) -> Color {
        return match self {
            ColourOption::Default() => default,
            ColourOption::Override(c) => c,
        };
    }
}

pub struct Button {
    pub rect: Rect,
    pub colour: Color,
    pub hover_colour: ColourOption,
    pub down_colour: ColourOption,
    pub text: String,
    pub font_size: f32,
    pub text_colour: Color,
    pub disabled: bool,
}

impl Button {
    pub fn new(rect: Rect, colour: Color, text: String, disabled: bool) -> Button {
        return Button {
            rect,
            colour,
            text,
            disabled,
            hover_colour: ColourOption::Default(),
            down_colour: ColourOption::Default(),
            font_size: 24.0,
            text_colour: BLACK,
        };
    }

    fn is_hovered(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_circ = Circle::new(mouse_x, mouse_y, 1.0);
        return mouse_circ.overlaps_rect(&self.rect);
    }

    fn is_down(&self) -> bool {
        let down = is_mouse_button_down(MouseButton::Left);
        return down && self.is_hovered();
    }

    pub fn draw(&self) {
        let hovered = self.is_hovered();
        let down = self.is_down();

        let hover_colour = Color::new(
            self.colour.r * 0.8,
            self.colour.g * 0.8,
            self.colour.b * 0.8,
            1.0,
        );
        let down_colour = Color::new(
            self.colour.r * 0.5,
            self.colour.g * 0.5,
            self.colour.b * 0.5,
            1.0,
        );
        let rect_colour = if self.disabled {
            self.colour
        } else {
            if down {
                self.down_colour.get_colour(down_colour)
            } else {
                if hovered {
                    self.hover_colour.get_colour(hover_colour)
                } else {
                    self.colour
                }
            }
        };

        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            rect_colour,
        );

        let text_x = self.rect.x + self.font_size / 5.0; // this is very arbitrary but ok
        let text_y = self.rect.y + self.rect.h / 2.0 + self.font_size / 4.0;
        draw_text(
            self.text.as_str(),
            text_x,
            text_y,
            self.font_size,
            self.text_colour,
        );
    }

    pub fn is_pressed(&self) -> bool {
        let pressed = is_mouse_button_pressed(MouseButton::Left);
        return pressed && self.is_hovered();
    }
}
