use anyhow::Result;
use snact_core::record::recorder::{Recorder, RecorderState};
use snact_core::record::Workflow;

pub fn run_start(name: Option<&str>, fmt: &str) -> Result<()> {
    if let Ok(Some(_)) = Recorder::load_state() {
        anyhow::bail!("Recording already in progress. Run `snact record stop` first.");
    }

    let name = name
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            format!("recording-{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap())
        });

    let state = RecorderState::new(&name);
    Recorder::save_state(&state)?;

    if fmt == "json" {
        println!("{}", serde_json::json!({"status": "ok", "action": "record_start", "name": name}));
    } else {
        println!("Recording started: {name}");
    }
    Ok(())
}

pub fn run_stop(fmt: &str) -> Result<()> {
    let state = Recorder::load_state()?.ok_or_else(|| {
        anyhow::anyhow!("No recording in progress. Run `snact record start` first.")
    })?;

    let workflow = Recorder::finalize(state);
    workflow.save()?;
    Recorder::clear_state()?;

    if fmt == "json" {
        println!("{}", serde_json::json!({
            "status": "ok",
            "action": "record_stop",
            "name": workflow.name,
            "steps": workflow.steps.len(),
        }));
    } else {
        println!(
            "Recording saved: {} ({} steps)",
            workflow.name,
            workflow.steps.len()
        );
    }
    Ok(())
}

pub fn run_list(fmt: &str) -> Result<()> {
    let workflows = Workflow::list()?;
    if fmt == "json" {
        println!("{}", serde_json::json!({"workflows": workflows}));
    } else if workflows.is_empty() {
        println!("No recorded workflows");
    } else {
        for name in workflows {
            println!("{name}");
        }
    }
    Ok(())
}

pub fn run_delete(name: &str, fmt: &str) -> Result<()> {
    Workflow::delete(name)?;
    if fmt == "json" {
        println!("{}", serde_json::json!({"status": "ok", "action": "record_delete", "name": name}));
    } else {
        println!("Workflow '{name}' deleted");
    }
    Ok(())
}
