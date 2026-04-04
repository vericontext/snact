use anyhow::Result;

pub async fn run(port: u16, name: &str, speed: f64, fmt: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        let workflow = snact_core::record::Workflow::load(name).map_err(|e| {
            anyhow::anyhow!("Failed to load workflow '{name}': {e}")
        })?;
        if fmt == "json" {
            println!("{}", serde_json::json!({
                "status": "dry_run",
                "action": "replay",
                "name": name,
                "steps": workflow.steps.len(),
            }));
        } else {
            println!("[dry-run] replay {name} ({} steps)", workflow.steps.len());
        }
        return Ok(());
    }

    let transport = snact_cdp::connect(port).await?;

    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    let result = snact_core::record::replay::execute(&transport, name, speed).await?;

    if fmt == "json" {
        println!("{}", serde_json::json!({
            "status": "ok",
            "action": "replay",
            "completed": result.completed,
            "total": result.total_steps,
            "warnings": result.warnings,
        }));
    } else {
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
    }

    Ok(())
}
