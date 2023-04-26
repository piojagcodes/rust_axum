use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    
    let hc = httpc_test::new_client("http:/localhost:8080")?;
    hc.do_get("/hello?name=Piotr").await?.print().await?;



    

    Ok(())
}
