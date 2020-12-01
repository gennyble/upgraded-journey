use fontdue::Font;
use png::{BitDepth, ColorType, Encoder};
use std::fs;

struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        let data = vec![0; width * height];

        Self {
            width,
            height,
            data
        }
    }

    fn from_parts(width: usize, height: usize, data: Vec<u8>) -> Self {
        if data.len() != width * height {
            panic!("Expected length to be {} but it's {}", width*height, data.len());
        }

        Self {
            width,
            height,
            data
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn as_vec(self) -> Vec<u8> {
        self.data
    }

    fn draw_img(&mut self, img: Image, off_x: usize, off_y: usize) {
        let img_data = img.data();
        for img_y in 0..img.height() {
            // current pixel y value
            let y = off_y + img_y;

            // If the pixel Y would be out of the bounds of our image...
            if y >= self.height {
                // It's over
                return;
            }

            for img_x in 0..img.width() {
                // Current pixel x value
                let x = off_x + img_x;

                // If the pixel X would be out of the bounds of our image...
                if x >= self.width {
                    // There might be more to do, but not in this loop
                    break;
                } else {
                    let img_index = img_y * img.width() + img_x;
                    let our_index = y * self.width() + x;

                    self.data[our_index] = img_data[img_index];
                }
            }
        }
    }
}

fn main() {
    let data = fs::read("AmandaFuckingSans.otf").expect("Failed to load font from file");
    let font = Font::from_bytes(data, Default::default()).expect("Failed to parse font");

    let px = 64.0;

    // An 'em' referes to the width of M historically, as it was usually the
    // widest character (and took up all the available horizontal space)
    let em = font.metrics('M', px).bounds.width;

    let line_metrics = font.horizontal_line_metrics(px).expect("Is this not a vertical font?");
    // This should the largest height a glpyh can have. ascent is positive (above baseline)
    // and descent is negative (below baseline).
    let max_height = line_metrics.ascent - line_metrics.descent;

    // Width/height, in characters, of the image
    let char_width = 16;
    let char_height = 8;

    let mut img = Image::new((char_width as f32 * em) as usize, (char_height as f32 * max_height) as usize);

    //            ASCII
    for index in 0..128u8 {
        let char_x = index % char_width;
        let char_y = index / char_width;

        let x = char_x as f32 * em;
        let y = char_y as f32 * em;

        let (metrics, bitmap) = font.rasterize(index as char, px);

        img.draw_img(
            Image::from_parts(metrics.width, metrics.height, bitmap),
            x as usize,
            y as usize
        );
    }

    let png_file = fs::File::create("raster.png").expect("Failed to create raster image file");
    let width = img.width() as u32;
    let height = img.height() as u32;

    let mut png = Encoder::new(png_file, width, height);
    png.set_color(ColorType::Grayscale);
    png.set_depth(BitDepth::Eight);

    let mut writer = png.write_header().expect("Failed to write PNG header");
    writer.write_image_data(img.data()).expect("Failed to write PNG data");
}
