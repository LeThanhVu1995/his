pub async fn send(to: &str, title: &str, body: &str) -> anyhow::Result<()> {
    let _ = (to, title, body); // mock HTTP call
    Ok(())
}
