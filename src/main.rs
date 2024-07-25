use std::fs;
use clap::Parser;
use anyhow::Result;

use headless_chrome::{
    Browser, 
    LaunchOptions,
    protocol::cdp::Page::CaptureScreenshotFormatOption, 
};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// URL para capturar a screenshot
    #[arg(short, long)]
    url: String,

    /// Largura da viewport
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Altura da viewport
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// Nome do arquivo de saída
    #[arg(short, long, default_value = "screenshot.png")]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    options.window_size = Some((args.width, args.height));

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    
    let png_data = tab
        .navigate_to(&args.url)?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    fs::write(&args.output, png_data)?;

    println!("Screenshot successfully created.");
    Ok(())
}
