use clap::Parser;
use figlet_rs::FIGfont;
use anyhow::{Result, anyhow};

/// IS4010 Final Project: A fun CLI tool to generate ASCII art from text.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text to convert into ASCII art
    #[arg(required = true)]
    text: String,

    /// Specify the font to use (e.g., 'standard', 'starwars')
    #[arg(short, long, default_value = "standard")]
    font: String,
}

fn main() -> Result<()> {
    // 1. Parse Command Line Arguments
    let args = Args::parse();

    // 2. Load the font
    // Note: figlet-rs 0.1.5 primarily supports the standard font
    // Font names are accepted for future compatibility
    let font_name = args.font.to_lowercase();
    if font_name != "standard" {
        eprintln!(
            "📝 Note: '{}' font requested. Using 'standard' font (figlet-rs 0.1.5 limitation).",
            args.font
        );
    }

    let font = FIGfont::standard().map_err(|e| anyhow!(e))?;
    
    // 3. Generate the ASCII art
    let figure = font
        .convert(&args.text)
        .ok_or_else(|| anyhow!("Failed to convert text to ASCII art"))?;

    // 4. Print the result
    println!("{}", figure);
    
    Ok(())
}