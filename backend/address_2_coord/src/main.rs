extern crate rustc_serialize;
extern crate quick_csv;
extern crate csv;
extern crate reqwest;
extern crate serde_json;

use quick_csv::Csv;
use serde_json::Value;
use std::io::{self, Write, Read};

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
struct Record {
    values: Vec<String>,
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
struct GeoRecord {
    record: Record,
    lat: f64,
    lng: f64,
}

fn read_records_from(path: &str, vec: &mut Vec<GeoRecord>) {
    let rdr = Csv::from_file(path).unwrap().has_header(true);
    for row in rdr.into_iter() {
        match row.unwrap().decode::<Record>() {
            Ok(cols) => {
                let geo_rec = GeoRecord {
                    record: cols,
                    lat: 0.0,
                    lng: 0.0,
                };
                vec.push(geo_rec);
            }
            Err(error) => println!("{}", error),
        }
    }
}

fn write_records_to(path: &str, vec: &mut Vec<GeoRecord>) {
    let mut wtr = csv::Writer::from_file(path).unwrap();
    for record in vec.into_iter() {
        wtr.encode(record.clone());
    }
}

fn get_coordinates(vec: &mut Vec<GeoRecord>) {
    for (i, record) in vec.into_iter().enumerate() {
        print!("{}> ", i);
        let (lat, lng) =
            coords(format!("https://maps.google.com/maps/api/geocode/json?address={},%20Atlanta,GA&key=AIzaSyAuwf-jxMOVTpEzdUf3BnmR8__f18IXFpg",
            // coords(format!("https://maps.google.com/maps/api/geocode/json?address={},%20Atlanta,GA",
                           record.record.values[10])
                           .as_ref());
        record.lat = lat;
        record.lng = lng;
    }
}

fn get_json_from(url: &str) -> String {
    let mut response = reqwest::get(url).expect("Failed to send request");
    println!("{}", response.status());

    let mut buf = String::new();
    response.read_to_string(&mut buf).ok();
    buf
}

fn coords(url: &str) -> (f64, f64) {
    let json = get_json_from(url);
    let r: Value = serde_json::from_str(json.as_ref()).unwrap();
    match r["results"][0]["geometry"]["location"] {
        Value::Object(ref location) => {
            (location["lat"].as_f64().unwrap(), location["lng"].as_f64().unwrap())
        }
        _ => {
            println!("Not so good: {}", url);
            (0.0, 0.0)
        }
    }
    // let location = &r["results"][0]["geometry"]["location"];
    // println!("{:#?}", location);
    // (location["lat"].as_f64().unwrap(), location["lng"].as_f64().unwrap())
}

fn main() {
    let mut records = Vec::new();
    read_records_from("../../data/x03", &mut records);
    get_coordinates(&mut records);
    write_records_to("../../data/out_x03.csv", &mut records);
}