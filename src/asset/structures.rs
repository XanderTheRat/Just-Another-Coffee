use clap::Parser;
use image::{DynamicImage, GenericImageView, imageops::FilterType};

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
pub struct Args {
    #[arg(short, long)]
    pub question: bool,

    #[arg(short, long)]
    pub no: bool,

    #[arg(short, long)]
    pub yes: bool,

    #[arg(short, long)]
    pub graduate_multicolor: bool,

    #[arg(short, long)]
    pub colored: bool,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long)]
    pub help: bool,
}

pub struct AsciiConverter {
    ascii_ramp: Vec<char>,
    block_ramp: Vec<char>,
}

impl AsciiConverter {
    pub fn new() -> Self {
        Self {
            ascii_ramp: vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            block_ramp: vec![' ', '░', '▒', '▓', '█'],
        }
    }

    /// In your terminal, a character is twice wide than it is tall.
    /// We have to resize all character to adapt the size of it is displayed in the terminal.
    fn resize_for_terminal(img: &DynamicImage, target_width: u32) -> DynamicImage {
        let half = 0.5;
        let (orig_w, orig_h) = img.dimensions();

        let target_height = ((orig_h as f32 / orig_w as f32) * target_width as f32 * half) as u32;

        img.resize_exact(target_width, target_height.max(1), FilterType::Lanczos3)
    }
    fn luminance(r: u8, g: u8, b: u8) -> u8 {
        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8
    }

    fn luma_to_char(luma: u8, ramp: &[char]) -> char {
        let index = (luma as usize * (ramp.len() - 1)) / 255;
        ramp[index]
    }
}
