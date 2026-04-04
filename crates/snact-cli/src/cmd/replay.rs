use anyhow::Result;

pub async fn run(port: u16, name: &str, speed: f64) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    // Enable Page events
    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    let result = snact_core::record::replay::execute(&transport, name, speed).await?;

    println!(
        "Replay complete: {}/{} steps",
        result.completed, result.total_steps
    );

    for warning in &result.warnings {
        eprintln!("warning: {warning}");
    }

    if let Some(step) = result.failed_step {
        eprintln!("Failed at step {step}");
    }

    Ok(())
}
