use anyhow::Result;
use json::{read_json, write_json};
use reqwest::get;
use serde_json::{json, Value};
mod json;

#[tokio::main]
async fn main() -> Result<()> {
    update().await?;
    calculate_diff()?;
    Ok(())
}

async fn update() -> Result<()> {
    let mut json = read_json("./result/result.json");
    let res = update_json().await?;
    json.as_array_mut().unwrap().push(res);
    write_json("./result/result.json", json);
    Ok(())
}

async fn update_json() -> Result<Value> {
    let res = get("https://ak.hypergryph.com/lynchpin/api/meta")
        .await?
        .json::<Value>()
        .await?;
    Ok(res)
}

fn calculate_diff() -> Result<()> {
    let json = read_json("./result/result.json");
    let mut tmp_vec = Vec::new();
    let iter = json.as_array().unwrap().iter();
    for value in iter {
        let progress = value["data"]["progress"].as_u64().unwrap();
        tmp_vec.push(progress);
    }
    let mut res = Vec::new();
    res[0] = json!({ "diff": tmp_vec[0] });
    for i in 1..tmp_vec.len() {
        res.push(json!({ "diff": tmp_vec[i] - tmp_vec[i - 1]}));
    }
    write_json("./result/diff.json", res);
    Ok(())
}
