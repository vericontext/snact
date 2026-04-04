use anyhow::Result;

pub async fn run(port: u16, url: Option<&str>, focus: Option<&str>, fmt: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    transport
        .send(&snact_cdp::commands::PageEnable {})
        .await?;

    let result = snact_core::snap::execute(&transport, url, focus).await?;

    if fmt == "json" {
        let json = serde_json::json!({
            "elements": result.element_map.elements,
            "count": result.element_count,
        });
        println!("{}", serde_json::to_string(&json)?);
    } else {
        println!("{}", result.output);
        eprintln!("({} elements)", result.element_count);
    }

    Ok(())
}
