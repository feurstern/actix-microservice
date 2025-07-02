#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("localhost:2121");
    hc.do_get("/hello").await?.print().await?;
}
