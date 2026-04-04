use anyhow::Result;

pub async fn run(port: u16, url: Option<&str>, focus: Option<&str>) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    // Enable Page events
    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    let result = snact_core::snap::execute(&transport, url, focus).await?;

    println!("{}", result.output);

    eprintln!("({} elements)", result.element_count);

    Ok(())
}
