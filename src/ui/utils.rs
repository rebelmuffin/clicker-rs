use macroquad::color::Color;

pub trait Add {
    fn add(&mut self, other: Self);
}

impl Add for Color {
    fn add(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}
