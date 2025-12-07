use anyhow::{anyhow, Result};
use clap::Parser;
use figlet_rs::FIGfont;

/// The IS4010 Final Project: A fun CLI tool to generate ASCII art from text.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The text to convert into ASCII art
    #[arg(required = true)]
    text: String,

    /// Specify the font to use (e.g., 'standard')
    #[arg(short, long, default_value = "standard")]
    font: String,
}

fn main() -> Result<()> {
    // 1. Parse Command Line Arguments
    let args = Args::parse();

    // 2. Load the specified font
    let font = match args.font.to_lowercase().as_str() {
        // For version 0.1.5, we primarily use the standard font
        "standard" | "slant" | "chunky" | "starwars" => {
            FIGfont::standard().map_err(|e| anyhow!(e))?
        }
        // Fallback for an unknown font name
        _ => {
            eprintln!(
                "Warning: Font '{}' not found. Using 'standard' font.",
                args.font
            );
            FIGfont::standard().map_err(|e| anyhow!(e))?
        }
    };

    // 3. Generate the ASCII art
    let figure = font
        .convert(&args.text)
        .ok_or_else(|| anyhow!("Failed to convert text to ASCII art"))?;

    // 4. Print the result
    println!("{}", figure);

    Ok(())
}
