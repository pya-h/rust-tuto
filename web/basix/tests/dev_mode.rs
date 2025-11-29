use anyhow::Result;

#[tokio::test]
async fn fetch_root() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn fetch_another() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/another").await?.print().await?;

    Ok(())
}