use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::num::ParseIntError;
use std::{fs::File, io::BufReader};

fn main() -> Result<()> {
    let mut data = Metadata::default();

    let pinout = BufReader::new(File::open("data/mcxa2/pinout.csv")?);
    let mut pinout_rdr = csv::Reader::from_reader(pinout);

    let memory_map = BufReader::new(File::open("data/mcxa2/memory-map.csv")?);
    let mut memory_map_rdr = csv::Reader::from_reader(memory_map);
    let all_peripherals = memory_map_rdr
        .deserialize::<MemoryMapRecord>()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().instance.to_uppercase())
        .filter(|name| !name.is_empty())
        .map(|name| Peripheral::new(name))
        .collect::<Vec<_>>();

    data.peripherals = all_peripherals;

    // Iterate over the data and extract all valid ALT0 and I/O SUPPLY
    for record in pinout_rdr
        .deserialize::<PinoutRecord>()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
    {
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

/// Custom function to deserialize a hex string into a u32
fn from_hex<'de, D>(deserializer: D) -> std::result::Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s = s.trim();

    // Remove optional "0x" or "0X" prefix
    let hex_str = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);

    u32::from_str_radix(hex_str, 16)
        .map_err(|e: ParseIntError| serde::de::Error::custom(format!("Invalid hex: {}", e)))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct MemoryMapRecord {
    #[serde(rename = "Peripheral description")]
    desc: String,
    #[serde(rename = "module_nickname")]
    nickname: String,
    #[serde(rename = "Peripheral instance")]
    instance: String,
    #[serde(rename = "Size (KB)")]
    size: u32,
    #[serde(rename = "Start address (hex)", deserialize_with = "from_hex")]
    start: u32,
    #[serde(rename = "End address (hex)", deserialize_with = "from_hex")]
    end: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PinoutRecord {
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
    // TODO: Complete signal table
    peripherals: Vec<Peripheral>,
}

impl Metadata {
    fn new(
        comment: String,
        chips: Vec<String>,
        pins: Vec<Pin>,
        peripherals: Vec<Peripheral>,
    ) -> Self {
        Self {
            schema: "./schema.json".to_string(),
            comment,
            chips,
            pins,
            peripherals,
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

#[derive(Serialize)]
struct Peripheral {
    name: String,
    signals: Vec<Signal>,
}

impl Peripheral {
    fn new(name: String) -> Self {
        Self {
            name,
            signals: vec![],
        }
    }
}

#[derive(Serialize)]
struct Signal {
    name: String,
    pins: Vec<Pin>,
}
