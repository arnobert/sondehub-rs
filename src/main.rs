use std::collections::HashMap;

use reqwest::header::ACCEPT_ENCODING;
use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    enum txmap_values {
        String(&'static str),
        Int(i32),
        Float(f32),
    }

    let mut txmap = HashMap::new();
    txmap.insert("dev", txmap_values::String("string"));
    txmap.insert("software_name", txmap_values::String("sondehub-rs"));
    txmap.insert("software_version", txmap_values::String("0.01"));
    txmap.insert("uploader_callsign", txmap_values::String("DL2SSB"));
    txmap.insert("time_received", txmap_values::String("2024-12-16T14:01:10.225Z"));
    txmap.insert("payload_callsign", txmap_values::String("DL2SSB-10"));
    txmap.insert("datetime", txmap_values::String("2024-12-16T14:01:10.225Z"));
    txmap.insert("lat", txmap_values::Float(51.4730));
    txmap.insert("lon", txmap_values::Float(7.2163));
    txmap.insert("alt", txmap_values::Int(23));


    println!("{:?}", txmap);

    let txclient = reqwest::Client::new();
    let txresponse = txclient.put("https://api.v2.sondehub.org/amateur/telemetry")
    //let txresponse = txclient.put("http://localhost:4711")
        .header(ACCEPT_ENCODING, "plain")
        .json(&[txmap])
        .send()
        .await?;

    println!("{:#?}", txresponse);
    println!("{:#?}", txresponse.headers());
    println!("{:#?}", txresponse.text().await?);


    Ok(())
}
