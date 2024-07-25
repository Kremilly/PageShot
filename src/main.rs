use std::fs;
use anyhow::Result;

use headless_chrome::{
    Browser, 
    LaunchOptions,
    protocol::cdp::Page::CaptureScreenshotFormatOption, 
};

fn main() -> Result<()> {
    let mut options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    options.window_size = Some((1920, 1080));

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    
    let png_data = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    fs::write("screenshot.png", png_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}
