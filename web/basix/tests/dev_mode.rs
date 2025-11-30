use anyhow::Result;
use serde_json::json;

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

#[tokio::test]
async fn fetch_sample_asset() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/sample.txt").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn do_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_post("/api/login", json!({"username": "root", "password": "toor"})).await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn do_invalid_login() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:8080").unwrap();

    let response = hc.do_post("/api/login", json!({"username": "what", "password": "ever"})).await?;
    
    assert_eq!(response.status(), 401);

    Ok(())
}
