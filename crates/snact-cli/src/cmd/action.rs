use anyhow::Result;

fn ok(fmt: &str, action: &str, extra: Option<(&str, &str)>) {
    if fmt == "json" {
        let mut obj = serde_json::json!({"status": "ok", "action": action});
        if let Some((k, v)) = extra {
            obj[k] = serde_json::Value::String(v.to_string());
        }
        println!("{}", obj);
    } else {
        if let Some((_, v)) = extra {
            println!("{v}");
        } else {
            println!("ok");
        }
    }
}

fn dry(fmt: &str, action: &str, args: serde_json::Value) {
    if fmt == "json" {
        println!(
            "{}",
            serde_json::json!({"status": "dry_run", "action": action, "args": args})
        );
    } else {
        println!("[dry-run] {action} {args}");
    }
}

pub async fn run_click(port: u16, ref_id: &str, fmt: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        dry(fmt, "click", serde_json::json!({"ref": ref_id}));
        return Ok(());
    }
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::click::execute(&transport, ref_id).await?;
    ok(fmt, "click", None);
    Ok(())
}

pub async fn run_fill(
    port: u16,
    ref_id: &str,
    value: &str,
    fmt: &str,
    dry_run: bool,
) -> Result<()> {
    if dry_run {
        dry(
            fmt,
            "fill",
            serde_json::json!({"ref": ref_id, "value": value}),
        );
        return Ok(());
    }
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::fill::execute(&transport, ref_id, value).await?;
    ok(fmt, "fill", None);
    Ok(())
}

pub async fn run_type(port: u16, ref_id: &str, text: &str, fmt: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        dry(
            fmt,
            "type",
            serde_json::json!({"ref": ref_id, "text": text}),
        );
        return Ok(());
    }
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::type_text::execute(&transport, ref_id, text).await?;
    ok(fmt, "type", None);
    Ok(())
}

pub async fn run_select(
    port: u16,
    ref_id: &str,
    value: &str,
    fmt: &str,
    dry_run: bool,
) -> Result<()> {
    if dry_run {
        dry(
            fmt,
            "select",
            serde_json::json!({"ref": ref_id, "value": value}),
        );
        return Ok(());
    }
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::select::execute(&transport, ref_id, value).await?;
    ok(fmt, "select", None);
    Ok(())
}

pub async fn run_scroll(
    port: u16,
    direction: &str,
    amount: Option<i64>,
    fmt: &str,
    dry_run: bool,
) -> Result<()> {
    if dry_run {
        dry(
            fmt,
            "scroll",
            serde_json::json!({"direction": direction, "amount": amount}),
        );
        return Ok(());
    }
    let transport = snact_cdp::connect(port).await?;
    snact_core::action::scroll::execute(&transport, direction, amount).await?;
    ok(fmt, "scroll", None);
    Ok(())
}

pub async fn run_screenshot(port: u16, output: Option<&str>, fmt: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;
    let path = snact_core::action::screenshot::execute(&transport, output).await?;
    ok(fmt, "screenshot", Some(("path", &path)));
    Ok(())
}

pub async fn run_wait(port: u16, condition: &str, fmt: &str) -> Result<()> {
    let transport = snact_cdp::connect(port).await?;

    let wait_condition = if condition == "navigation" {
        snact_core::action::wait::WaitCondition::Navigation
    } else if let Ok(ms) = condition.parse::<u64>() {
        snact_core::action::wait::WaitCondition::Timeout(ms)
    } else {
        snact_core::action::wait::WaitCondition::Selector(condition)
    };

    snact_core::action::wait::execute(&transport, wait_condition).await?;
    ok(fmt, "wait", None);
    Ok(())
}
