use std::collections::HashMap;

use reqwest::header::ACCEPT_ENCODING;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, SecondsFormat};

const UPLOADER_CALLSIGN: &str = "DL2SSB";
const SW_VERSION: &str = env!("CARGO_PKG_VERSION");
const SW_NAME: &str = env!("CARGO_PKG_NAME");


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dt = Local::now().to_utc()
        .to_rfc3339_opts(SecondsFormat::Secs, true);

    //
    // GET
    //
    //let mut res = reqwest::get("http://api.v2.sondehub.org/amateur/telemetry/DO3GU-18?last=50000").await?;
    //println!("{:#?}", res.status());

    //let headz = res.headers();
    //println!("{:#?}", headz);

    //let data = res.text().await?;

    //let pdata = serde_json::from_str::<Value>(&data);
    //println!("{:#?}", pdata);

    //
    // PUT
    //

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum TxmapValues<'a> {
        String(&'a str),
        Int(i32),
        Float(f32),
    }

    let mut txmap = HashMap::new();
    txmap.insert("dev", TxmapValues::String("string"));
    txmap.insert("software_name", TxmapValues::String(SW_NAME));
    txmap.insert("software_version", TxmapValues::String(SW_VERSION));
    txmap.insert("uploader_callsign", TxmapValues::String(UPLOADER_CALLSIGN));
    txmap.insert("time_received", TxmapValues::String("2025-01-08T22:01:10.225Z"));
    txmap.insert("payload_callsign", TxmapValues::String("DL2SSB-10"));
    txmap.insert("datetime", TxmapValues::String(&dt));
    txmap.insert("lat", TxmapValues::Float(51.4730));
    txmap.insert("lon", TxmapValues::Float(7.2163));
    txmap.insert("alt", TxmapValues::Int(23));


    println!("{:?}", txmap);

    let txclient = reqwest::Client::new();
    let txresponse = txclient.put("https://api.v2.sondehub.org/amateur/telemetry")
    //let txresponse = txclient.put("http://localhost:4711")
        .header(ACCEPT_ENCODING, "plain")
        .json(&[txmap])
        .send()
        .await?;

    let resp = txresponse.text().await?;
    let resp_fmt: Value = serde_json::from_str(&resp)?;

    //println!("{:#?}", txresponse);
    //println!("{:#?}", txresponse.headers());
    println!("{:#?}", resp_fmt);


    Ok(())
}
