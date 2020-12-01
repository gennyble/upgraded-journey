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
    let mut img1 = Image::new(64, 64);

    let img2_data: Vec<u8> = vec![0; 32 * 32]
        .into_iter()
        .map(|x: u8| 255u8)
        .collect::<Vec<u8>>();

    let img2 = Image::from_parts(32, 32, img2_data);

    img1.draw_img(img2, 16, 16);

    /*let data = fs::read("AmandaFuckingSans.otf").expect("Failed to load font from file");
    let font = Font::from_bytes(data, Default::default()).expect("Failed to parse font");

    let px = 16.0;

    // An 'em' referes to the width of M historically, as it was usually the
    // widest character (and took up all the available horizontal space)
    let em = font.metrics('M', px).bounds.width;

    let vert_metric = font.vertical_line_metrics(px).expect("Is this not a horizontal font?");
    // This should the largest height a glpyh can have. ascent is positive (above baseline)
    // and descent is negative (below baseline).
    let max_height = vert_metric.ascent - vert_metric.descent;

    // Width/height, in characters, of the image
    let char_width = 16.0;
    let char_height = 8.0;

    let img = Image::new((char_width * em) as u16, (char_height * max_height) as u16);*/

    let png_file = fs::File::create("raster.png").expect("Failed to create raster image file");
    let width = img1.width() as u32;
    let height = img1.height() as u32;

    let mut png = Encoder::new(png_file, width, height);
    png.set_color(ColorType::Grayscale);
    png.set_depth(BitDepth::Eight);

    let mut writer = png.write_header().expect("Failed to write PNG header");
    writer.write_image_data(img1.data()).expect("Failed to write PNG data");
}
