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


#[tokio::test]
async fn greet_paya() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/greet?name=paya").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn greet_paya_n_times() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/greet/10?name=paya").await?.print().await?;

    Ok(())
}