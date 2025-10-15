use std::fs;
use clap::Parser;
use anyhow::Result;

use headless_chrome::{
    Browser,
    LaunchOptions,
    protocol::cdp::Page::CaptureScreenshotFormatOption,
    protocol::cdp::Emulation,
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

    /// Output format: png, jpeg, or webp
    #[arg(long, default_value = "png")]
    format: String,

    /// Quality for JPEG/WebP (0-100, higher is better quality)
    #[arg(long, default_value_t = 85)]
    quality: u8,

    /// Device scale factor / pixel ratio (1.0 = standard, 2.0 = Retina 2x, 3.0 = 3x)
    #[arg(long, default_value_t = 1.0)]
    scale: f64,

    /// Suppress success message
    #[arg(short, long)]
    silent: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse format and quality
    let format = match args.format.to_lowercase().as_str() {
        "jpeg" | "jpg" => CaptureScreenshotFormatOption::Jpeg,
        "webp" => CaptureScreenshotFormatOption::Webp,
        _ => CaptureScreenshotFormatOption::Png,
    };

    // Quality only applies to JPEG and WebP, clamp to valid range
    let quality = if matches!(format, CaptureScreenshotFormatOption::Jpeg | CaptureScreenshotFormatOption::Webp) {
        Some(args.quality.clamp(0, 100) as u32)
    } else {
        None
    };

    // Clamp scale factor to valid range (1.0 to 3.0)
    let scale = args.scale.clamp(1.0, 3.0);

    let mut options = LaunchOptions::default_builder()
        .headless(true)
        .build()
        .map_err(|_| anyhow::anyhow!("Couldn't find appropriate Chrome binary."))?;

    options.window_size = Some((args.width, args.height));

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(&args.url)?
        .wait_until_navigated()?;

    // Determine final dimensions based on full_page flag
    let (final_width, final_height) = if args.full_page {
        // Get full page dimensions
        let full_width = tab.evaluate(
            "Math.max(document.body.scrollWidth, document.documentElement.scrollWidth)",
            false
        )?
        .value
        .and_then(|v| v.as_f64())
        .ok_or_else(|| anyhow::anyhow!("Failed to get page width from JavaScript evaluation"))? as u32;

        let full_height = tab.evaluate(
            "Math.max(document.body.scrollHeight, document.documentElement.scrollHeight)",
            false
        )?
        .value
        .and_then(|v| v.as_f64())
        .ok_or_else(|| anyhow::anyhow!("Failed to get page height from JavaScript evaluation"))? as u32;

        // Set viewport to full page dimensions
        tab.set_bounds(headless_chrome::types::Bounds::Normal {
            left: Some(0),
            top: Some(0),
            width: Some(full_width as f64),
            height: Some(full_height as f64),
        })?;

        (full_width, full_height)
    } else {
        (args.width, args.height)
    };

    // Set device metrics to ensure exact viewport dimensions
    tab.call_method(Emulation::SetDeviceMetricsOverride {
        width: final_width,
        height: final_height,
        device_scale_factor: scale,
        mobile: false,
        scale: None,
        screen_width: None,
        screen_height: None,
        position_x: None,
        position_y: None,
        dont_set_visible_size: None,
        screen_orientation: None,
        viewport: None,
        display_feature: None,
        device_posture: None,
    })?;

    // Give the page a moment to adjust if we resized
    if args.full_page {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let screenshot_data = tab.capture_screenshot(
        format,
        quality,
        None,
        true
    )?;

    // Check for empty screenshot data (indicates capture failure)
    if screenshot_data.is_empty() {
        anyhow::bail!(
            "Screenshot capture failed (empty data). This may happen with very large dimensions. \
             Try reducing scale factor or viewport size. Current: {}x{} at {}x scale = {}x{} pixels",
            final_width, final_height, scale,
            (final_width as f64 * scale) as u32,
            (final_height as f64 * scale) as u32
        );
    }

    fs::write(&args.output, screenshot_data)?;

    if !args.silent {
        println!("Screenshot saved to: {}", args.output);
    }

    Ok(())
}
