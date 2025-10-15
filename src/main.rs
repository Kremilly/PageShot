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
    /// URL to capture the screenshot
    #[arg(short, long)]
    url: String,

    /// Width of the viewport
    #[arg(long, default_value_t = 1920)]
    width: u32,

    /// Height of the viewport
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// Output file name
    #[arg(short, long, default_value = "screenshot.png")]
    output: String,

    /// Capture full page screenshot (beyond viewport)
    #[arg(short, long)]
    full_page: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut options = LaunchOptions::default_builder()
        .headless(true)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    options.window_size = Some((args.width, args.height));

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(&args.url)?
        .wait_until_navigated()?;

    let screenshot_data = if args.full_page {
        // Get full page dimensions
        let full_width = tab.evaluate(
            "Math.max(document.body.scrollWidth, document.documentElement.scrollWidth)",
            false
        )?.value.unwrap().as_f64().unwrap() as u32;

        let full_height = tab.evaluate(
            "Math.max(document.body.scrollHeight, document.documentElement.scrollHeight)",
            false
        )?.value.unwrap().as_f64().unwrap() as u32;

        // Set viewport to full page dimensions
        tab.set_bounds(headless_chrome::types::Bounds::Normal {
            left: Some(0),
            top: Some(0),
            width: Some(full_width as f64),
            height: Some(full_height as f64),
        })?;

        // Give the page a moment to adjust to new dimensions
        std::thread::sleep(std::time::Duration::from_millis(500));

        tab.capture_screenshot(
            CaptureScreenshotFormatOption::Png,
            None,
            None,
            true
        )?
    } else {
        tab.capture_screenshot(
            CaptureScreenshotFormatOption::Png,
            None,
            None,
            true
        )?
    };

    fs::write(&args.output, screenshot_data)?;

    println!("Screenshot successfully created.");
    Ok(())
}
