use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/").await?.print().await?;
    hc.do_get("/get-gas-price").await?.print().await?;
    hc.do_get("/get-block-number").await?.print().await?;
    hc.do_get("/get-balance/0x8878D966b24458b4F3C27DB7aAC2bf0154A6D5dA")
        .await?
        .print()
        .await?;

    Ok(())
}
