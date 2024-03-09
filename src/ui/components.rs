use macroquad::prelude::*;

#[allow(dead_code)] // is this even good practice
#[derive(Copy, Clone)]
pub enum ColourOption {
    Default(),
    Override(Color),
}

impl Default for ColourOption {
    fn default() -> Self {
        ColourOption::Default()
    }
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

    text: String,
    font_size: f32,
    text_colour: Color,
    text_padding: f32,

    pub disabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            rect: Default::default(),
            colour: Default::default(),
            hover_colour: Default::default(),
            down_colour: Default::default(),
            text: Default::default(),
            font_size: 24.0,
            text_colour: BLACK,
            text_padding: 10.0,
            disabled: false,
        }
    }
}

impl Button {
    pub fn new(
        mut rect: Rect,
        colour: Color,
        text: String,
        disabled: bool,
        auto_width: bool,
    ) -> Button {
        if auto_width {
            let default = Button::default();
            let text_width = measure_text(text.as_str(), None, default.font_size as u16, 1.0).width;
            rect.w = text_width + default.text_padding * 2.0;
        }

        return Button {
            rect,
            colour,
            text,
            disabled,
            ..Default::default()
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

        // Center align the text
        // #TODO implement multiple alignment types
        let text_width = measure_text(self.text.as_str(), None, self.font_size as u16, 1.0).width;
        let button_width = self.rect.w;
        let text_x = self.rect.x + button_width / 2.0 - text_width / 2.0;

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
