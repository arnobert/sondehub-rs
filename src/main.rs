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

    impl<'a> From<&'a str> for TxmapValues<'a> {
        fn from(value: &'a str) -> Self {
            TxmapValues::String(value)
        }
    }

    impl<'a> From<i32> for TxmapValues<'a> {
        fn from(value: i32) -> Self {
            TxmapValues::Int(value)
        }
    }

    impl<'a> From<f32> for TxmapValues<'a> {
        fn from(value: f32) -> Self {
            TxmapValues::Float(value)
        }
    }


    let mut txmap: HashMap<&str, TxmapValues> = HashMap::new();
    txmap.insert("dev", "string".into());
    txmap.insert("software_name", SW_NAME.into());
    txmap.insert("software_version", SW_VERSION.into());
    txmap.insert("uploader_callsign", UPLOADER_CALLSIGN.into());
    txmap.insert("time_received", "2025-01-08T22:01:10.225Z".into());
    txmap.insert("payload_callsign", "DL2SSB-10".into());
    txmap.insert("datetime", (&*dt).into());
    txmap.insert("lat", 51.4730.into());
    txmap.insert("lon", 7.2163.into());
    txmap.insert("alt", 23.into());


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
