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
struct Fade(f32);
impl FromStr for Fade {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse::<f32>().map_err(|_| "Invalide fade")?;
        if v > 1.0 || v < 0.0 {
            return Err("Allowed values are 0.0 to 1.0".to_string());
        }
        Ok(Fade(v))
    }
}
impl fmt::Display for Fade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
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
    /// Control how transparent the overlay should be at the bottom
    #[arg(long, default_value_t=Fade(1.0))]
    fade: Fade,
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
        overlay::generate_from_url(url, out_filename, gradient_variant, args.fade.0);
    } else if let Some(in_filename) = args.input_file {
        overlay::generate(in_filename, out_filename, gradient_variant, args.fade.0)
    }
    println!("Done")
}
