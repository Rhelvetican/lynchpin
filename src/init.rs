use std::fs::DirBuilder;

use anyhow::{Ok, Result};
use serde_json::json;

use crate::json::write_json;

pub fn init() -> Result<()> {
    let cur = json!([
        {
            "code": 0,
            "data": {
                "progress": 15
            },
            "msg": ""
        },
        {
            "code": 0,
            "data": {
                "progress": 24
            },
            "msg": ""
        },
        {
            "code": 0,
            "data": {
                "progress": 33
            },
            "msg": ""
        },
        {
            "code": 0,
            "data": {
                "progress": 34
            },
            "msg": ""
        }
    ]);
    DirBuilder::new().recursive(true).create("./result")?;
    write_json("result/result.json", cur);
    Ok(())
}
