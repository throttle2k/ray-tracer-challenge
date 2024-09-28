use colo_rs::colors::Color;

use crate::canvas::Canvas;

struct Header {
    version: String,
    width: usize,
    height: usize,
    max_color: u8,
}

struct PixelData {
    width: usize,
    data: Vec<Color>,
}

pub struct PPM {
    header: Header,
    pixel_data: PixelData,
}

impl ToString for Header {
    fn to_string(&self) -> String {
        format!(
            "{}\n{} {}\n{}\n",
            self.version, self.width, self.height, self.max_color
        )
    }
}

impl ToString for PixelData {
    fn to_string(&self) -> String {
        let s = self
            .data
            .iter()
            .map(|c| c.as_255_string())
            .collect::<Vec<String>>();
        let mut as_string: Vec<String> = Vec::new();
        let mut string = String::new();
        let mut count = 0;
        let mut seventies_to_subtract = 0;
        s.iter().for_each(|s| {
            s.split(' ').for_each(|sub| {
                if string.len() + sub.len() - seventies_to_subtract * 70 > 70 {
                    seventies_to_subtract += 1;
                    string = string.trim().to_string();
                    string.push('\n');
                }
                string.push_str(sub);
                string.push(' ');
            });
            count += 1;
            if count == self.width {
                as_string.push(string.trim().to_string().clone());
                string = String::new();
                seventies_to_subtract = 0;
                count = 0;
            }
        });
        as_string.join("\n")
    }
}

impl ToString for PPM {
    fn to_string(&self) -> String {
        let mut s = self.header.to_string();
        s.push_str(&self.pixel_data.to_string());
        s.push('\n');
        s
    }
}

impl From<Canvas> for PPM {
    fn from(canvas: Canvas) -> Self {
        Self {
            header: Header {
                version: String::from("P3"),
                width: canvas.width(),
                height: canvas.height(),
                max_color: 255,
            },
            pixel_data: PixelData {
                width: canvas.width(),
                data: canvas.pixels().clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = PPM::from(canvas);
        assert_eq!(
            ppm.header.to_string(),
            r#"P3
5 3
255
"#
        )
    }

    #[test]
    fn contructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.0, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = PPM::from(canvas);
        assert_eq!(
            ppm.pixel_data.to_string(),
            r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"#
        )
    }

    #[test]
    fn splitting_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);
        canvas
            .pixels_mut()
            .iter_mut()
            .for_each(|c| *c = Color::new(1.0, 0.8, 0.6));
        let ppm = PPM::from(canvas);
        assert_eq!(
            ppm.pixel_data.to_string(),
            r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"#
        );
    }
}
