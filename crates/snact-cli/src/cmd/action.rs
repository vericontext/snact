use anyhow::Result;

pub async fn run_click(port: u16, ref_id: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::click::execute(&transport, ref_id).await?;
    println!("ok");
    Ok(())
}

pub async fn run_fill(port: u16, ref_id: &str, value: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::fill::execute(&transport, ref_id, value).await?;
    println!("ok");
    Ok(())
}

pub async fn run_type(port: u16, ref_id: &str, text: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::type_text::execute(&transport, ref_id, text).await?;
    println!("ok");
    Ok(())
}

pub async fn run_select(port: u16, ref_id: &str, value: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::select::execute(&transport, ref_id, value).await?;
    println!("ok");
    Ok(())
}

pub async fn run_scroll(port: u16, direction: &str, amount: Option<i64>) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::scroll::execute(&transport, direction, amount).await?;
    println!("ok");
    Ok(())
}

pub async fn run_screenshot(port: u16, output: Option<&str>) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    let path = snact_core::action::screenshot::execute(&transport, output).await?;
    println!("{path}");
    Ok(())
}

pub async fn run_wait(port: u16, condition: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    let wait_condition = if condition == "navigation" {
        snact_core::action::wait::WaitCondition::Navigation
    } else if let Ok(ms) = condition.parse::<u64>() {
        snact_core::action::wait::WaitCondition::Timeout(ms)
    } else {
        snact_core::action::wait::WaitCondition::Selector(condition)
    };

    snact_core::action::wait::execute(&transport, wait_condition).await?;
    println!("ok");
    Ok(())
}
