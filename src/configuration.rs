use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(name = "yair")]
#[command(author = "3d0")]
#[command(about = "Yet Another Image Resizer")]
pub struct Config {
    /// Path of the image to resize
    #[arg(short, long)]
    pub image_path: String,

    /// Resize percentage
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..100))]
    pub percentage: u32,
}
