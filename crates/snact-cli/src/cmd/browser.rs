use anyhow::Result;

pub fn run_launch(port: u16, headless: bool) -> Result<()> {
    let browser = snact_cdp::ManagedBrowser::launch(port, headless)?;
    println!("Chrome launched on port {}", browser.port());

    // Keep the process alive
    println!("Press Ctrl+C to stop");
    std::thread::park();

    Ok(())
}
