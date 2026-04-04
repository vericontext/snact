use anyhow::Result;

pub async fn run_save(port: u16, name: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    // Enable Page events for navigation detection
    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    let profile = snact_core::session::capture_session(&transport, name).await?;
    println!("Session '{}' saved ({} cookies)", name, profile.cookies.len());
    Ok(())
}

pub async fn run_load(port: u16, name: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    // Enable Page events for navigation detection
    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    snact_core::session::restore_session(&transport, name).await?;
    println!("Session '{name}' restored");
    Ok(())
}

pub fn run_list() -> Result<()> {
    let sessions = snact_core::session::SessionProfile::list()?;
    if sessions.is_empty() {
        println!("No saved sessions");
    } else {
        for name in sessions {
            println!("{name}");
        }
    }
    Ok(())
}

pub fn run_delete(name: &str) -> Result<()> {
    snact_core::session::SessionProfile::delete(name)?;
    println!("Session '{name}' deleted");
    Ok(())
}
