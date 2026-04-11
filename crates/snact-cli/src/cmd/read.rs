use anyhow::Result;

pub async fn run(
    port: u16,
    url: Option<&str>,
    focus: Option<&str>,
    fmt: &str,
    lang: &str,
    max_lines: usize,
) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    let result = snact_core::read::execute(&transport, url, focus, lang, max_lines).await?;

    match fmt {
        "json" => {
            let json = serde_json::json!({
                "content": result.output,
                "line_count": result.line_count,
            });
            println!("{}", serde_json::to_string(&json)?);
        }
        _ => {
            println!("{}", result.output);
            eprintln!("({} lines)", result.line_count);
        }
    }

    Ok(())
}
