pub async fn send(to: &str, body: &str) -> anyhow::Result<()> {
    let _ = (to, body); // mock HTTP call
    Ok(())
}
