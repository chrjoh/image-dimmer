#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use clap::Parser;
use std::fmt;
use std::str::FromStr;

mod overlay;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
enum GradientType {
    Dominant,
    DominantBottom,
    UserDefined,
}
#[derive(Debug, Clone)]
struct Rgb(u8, u8, u8);

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("Expected format: R,G,B".into());
        }

        let r = parts[0]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Invalid R value")?;
        let g = parts[1]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Invalid G value")?;
        let b = parts[2]
            .trim()
            .parse::<u8>()
            .map_err(|_| "Invalid B value")?;

        Ok(Rgb(r, g, b))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// url to fect image from
    #[arg(long)]
    url: Option<String>,
    /// read input image from file
    #[arg(long)]
    input_file: Option<String>,
    /// file to save the image to
    #[arg(long)]
    output_file: String,
    /// what gradient color to be used, dominant, dominat-bottom from image bottom or user-supplied
    /// with user-supplied the rgb is mandatory
    #[arg(value_enum,long, default_value_t=GradientType::DominantBottom)]
    gradient_variant: GradientType,
    /// R,G,B color values, 0-255,0-255,0-255
    #[arg(long)]
    rgb: Option<Rgb>,
}
fn main() {
    let args = Args::parse();
    let out_filename = args.output_file;
    let gradient_variant = match args.gradient_variant {
        GradientType::Dominant => overlay::GradientColorType::Dominant,
        GradientType::DominantBottom => overlay::GradientColorType::DominantBottom,
        GradientType::UserDefined => {
            if let Some(rgb) = args.rgb {
                overlay::GradientColorType::UserSelected(rgb.0, rgb.1, rgb.2)
            } else {
                panic!("Missing mandatory rgb values for user defined gradient")
            }
        }
    };
    if let Some(url) = args.url {
        overlay::generate_from_url(url, out_filename, gradient_variant);
    } else if let Some(in_filename) = args.input_file {
        overlay::generate(in_filename, out_filename, gradient_variant)
    }
    println!("Done")
}
