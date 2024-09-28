use colo_rs::colors::Color;

#[derive(Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let idx = self.xy_to_idx(x, y);
        self.pixels[idx] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.xy_to_idx(x, y)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.iter().all(|p| *p == Color::black()), true);
    }

    #[test]
    fn writing_pixel_to_a_canvas() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(2, 3, Color::red());
        assert_eq!(canvas.pixel_at(2, 3), Color::red());
        assert_eq!(canvas.pixel_at(1, 3), Color::black());
    }
}
