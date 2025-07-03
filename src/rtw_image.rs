//rtw_stb_image.rs
use image::codecs::hdr::HdrDecoder;
use image::{DynamicImage, GenericImageView, ImageReader, Pixel};
use std::{env, path::PathBuf};

pub struct RtwImage {
    pub width: usize,
    pub height: usize,
    pub fdata: Vec<f32>, // 每像素3个 float32，线性空间 [0.0, 1.0]
    pub bdata: Vec<u8>,  // 同样是 RGB 排列，8-bit 色彩分量 [0,255]
}

impl RtwImage {
    pub fn new(filename: &str) -> Option<Self> {
        let paths = Self::search_paths(filename);

        for path in paths {
            if let Ok(reader) = ImageReader::open(&path) {
                if let Ok(img) = reader.decode() {
                    return Some(Self::from_image(img));
                }
            }
        }

        eprintln!("ERROR: Could not load image file '{}'", filename);
        None
    }

    fn from_image(img: DynamicImage) -> Self {
        let img_rgb = img.to_rgb32f();
        let (width, height) = img_rgb.dimensions();
        let mut fdata = Vec::with_capacity((width * height * 3) as usize);
        let mut bdata = Vec::with_capacity((width * height * 3) as usize);

        for pixel in img_rgb.pixels() {
            let [r, g, b] = pixel.0;
            fdata.push(r);
            fdata.push(g);
            fdata.push(b);

            bdata.push(Self::float_to_byte(r));
            bdata.push(Self::float_to_byte(g));
            bdata.push(Self::float_to_byte(b));
        }

        Self {
            width: width as usize,
            height: height as usize,
            fdata,
            bdata,
        }
    }

    fn search_paths(filename: &str) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        if let Ok(imagedir) = env::var("RTW_IMAGES") {
            paths.push(PathBuf::from(imagedir).join(filename));
        }

        let candidates = [
            ".",
            "images",
            "../images",
            "../../images",
            "../../../images",
            "../../../../images",
            "../../../../../images",
            "../../../../../../images",
        ];

        for base in candidates {
            paths.push(PathBuf::from(base).join(filename));
        }

        paths
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        if self.bdata.is_empty() {
            return [255, 0, 255]; // Magenta fallback
        }

        let x = x.clamp(0, (self.width - 1) as i32);
        let y = y.clamp(0, (self.height - 1) as i32);

        let idx = (y as usize * self.width + x as usize) * 3;
        [self.bdata[idx], self.bdata[idx + 1], self.bdata[idx + 2]]
    }

    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (256.0 * value) as u8
        }
    }
}
