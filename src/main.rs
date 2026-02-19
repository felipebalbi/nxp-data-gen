use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let mut data = Metadata::default();

    let pinout = BufReader::new(File::open("data/mcxa2/pinout.csv").expect("cannot open file"));
    let mut rdr = csv::Reader::from_reader(pinout);

    let all_records = rdr
        .deserialize()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<Record>>();

    // Iterate over the data and extract all valid ALT0 and I/O SUPPLY
    for record in all_records {
        let supply = record.supply;
        let gpio = record.alt0;

        match (supply, gpio) {
            (Some(s), Some(p)) => data.pins.push(Pin::new(p.to_owned(), s.to_owned())),
            _ => {}
        }
    }

    let json = serde_json::to_string_pretty(&data)?;
    println!("{}", json);

    Ok(())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Record {
    #[serde(rename = "MCXA26x/A25x/A18x/A17x\nLQFP144")]
    lqfp144_pin_number: Option<u32>,
    #[serde(rename = "MCXA26x/A25x/A18x/A17x\n LQFP144 Pin Name")]
    lqfp144_pin_name: Option<String>,
    #[serde(rename = "MCXA26x/A25x/A18x/A17x\nWFBGA169")]
    wfbga169_pin_coord: Option<String>,
    #[serde(rename = "MCXA26x/A25x/A18x/A17x\nWFBGA169 Pin Name")]
    wfbga169_pin_name: Option<String>,
    #[serde(rename = " MCXA26x/A25x/A18x/A17x\nLQFP100")]
    lqfp100_pin_number: Option<u32>,
    #[serde(rename = " MCXA26x/A25x/A18x/A17x \nLQFP100 Pin Name")]
    lqfp100_pin_name: Option<String>,
    #[serde(rename = " MCXA26x/A25x/A18x/A17x\nLQFP64")]
    lqfp64_pin_number: Option<u32>,
    #[serde(rename = " MCXA26x/A25x/A18x/A17x\nLQFP64 Pin Name")]
    lqfp64_pin_name: Option<String>,
    #[serde(rename = "I/O Supply")]
    supply: Option<String>,
    #[serde(rename = "Default")]
    default: Option<String>,
    #[serde(rename = "ISP")]
    isp: Option<String>,
    #[serde(rename = "ANALOG")]
    analog: Option<String>,
    #[serde(rename = "ALT0")]
    alt0: Option<String>,
    #[serde(rename = "ALT1")]
    alt1: Option<String>,
    #[serde(rename = "ALT2")]
    alt2: Option<String>,
    #[serde(rename = "ALT3")]
    alt3: Option<String>,
    #[serde(rename = "ALT4")]
    alt4: Option<String>,
    #[serde(rename = "ALT5")]
    alt5: Option<String>,
    #[serde(rename = "ALT6")]
    alt6: Option<String>,
    #[serde(rename = "ALT7")]
    alt7: Option<String>,
    #[serde(rename = "ALT8")]
    alt8: Option<String>,
    #[serde(rename = "ALT9")]
    alt9: Option<String>,
    #[serde(rename = "ALT10")]
    alt10: Option<String>,
    #[serde(rename = "ALT11")]
    alt11: Option<String>,
    #[serde(rename = "ALT12")]
    alt12: Option<String>,
    #[serde(rename = "VDD_SYS")]
    vdd_sys: Option<String>,
    #[serde(rename = "Pad type")]
    pad_type: Option<String>,
}

#[derive(Serialize)]
struct Metadata {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "$comment")]
    comment: String,
    // Should, somehow, be extracted from vendor data files
    chips: Vec<String>,
    pins: Vec<Pin>,
    // todo
    // peripherals: Vec<Peripheral>,
}

impl Metadata {
    fn new(comment: String, chips: Vec<String>, pins: Vec<Pin>) -> Self {
        Self {
            schema: "./schema.json".to_string(),
            comment,
            chips,
            pins,
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(
            "MCXA1xx/MCXA2xx metadata".to_string(),
            vec![
                "MCXA175VLQ".to_string(),
                "MCXA175VLL".to_string(),
                "MCXA175VLH".to_string(),
                "MCXA175VPN".to_string(),
                "MCXA176VLQ".to_string(),
                "MCXA176VLL".to_string(),
                "MCXA176VLH".to_string(),
                "MCXA176VPN".to_string(),
                "MCXA185VLQ".to_string(),
                "MCXA185VLL".to_string(),
                "MCXA185VLH".to_string(),
                "MCXA185VPN".to_string(),
                "MCXA186VLQ".to_string(),
                "MCXA186VLL".to_string(),
                "MCXA186VLH".to_string(),
                "MCXA186VPN".to_string(),
                "MCXA255VPN".to_string(),
                "MCXA255VLH".to_string(),
                "MCXA255VLL".to_string(),
                "MCXA255VLQ".to_string(),
                "MCXA256VPN".to_string(),
                "MCXA256VLH".to_string(),
                "MCXA256VLL".to_string(),
                "MCXA256VLQ".to_string(),
                "MCXA265VPN".to_string(),
                "MCXA265VLH".to_string(),
                "MCXA265VLL".to_string(),
                "MCXA265VLQ".to_string(),
                "MCXA266VPN".to_string(),
                "MCXA266VLH".to_string(),
                "MCXA266VLL".to_string(),
                "MCXA266VLQ".to_string(),
            ],
            vec![],
        )
    }
}

#[derive(Serialize)]
struct Pin {
    name: String,
    supply: String,
}

impl Pin {
    fn new(name: String, supply: String) -> Self {
        Self { name, supply }
    }
}
