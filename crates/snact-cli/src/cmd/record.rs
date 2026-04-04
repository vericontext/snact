use anyhow::Result;
use snact_core::record::recorder::{Recorder, RecorderState};
use snact_core::record::Workflow;

pub fn run_start(name: Option<&str>) -> Result<()> {
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
    println!("Recording started: {name}");
    Ok(())
}

pub fn run_stop() -> Result<()> {
    let state = Recorder::load_state()?.ok_or_else(|| {
        anyhow::anyhow!("No recording in progress. Run `snact record start` first.")
    })?;

    let workflow = Recorder::finalize(state);
    workflow.save()?;
    Recorder::clear_state()?;

    println!(
        "Recording saved: {} ({} steps)",
        workflow.name,
        workflow.steps.len()
    );
    Ok(())
}

pub fn run_list() -> Result<()> {
    let workflows = Workflow::list()?;
    if workflows.is_empty() {
        println!("No recorded workflows");
    } else {
        for name in workflows {
            println!("{name}");
        }
    }
    Ok(())
}

pub fn run_delete(name: &str) -> Result<()> {
    Workflow::delete(name)?;
    println!("Workflow '{name}' deleted");
    Ok(())
}
