extern crate record;
extern crate kernel;
extern crate clustering;
extern crate cogset;
extern crate geojson;
extern crate serde_json;

use std::collections::Bound::Included;
use std::io::prelude::*;
use std::fs::File;
use cogset::{Dbscan, Optics, BruteScan, Euclid};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use serde_json::map::Map;
use serde_json::value;
use record::GeoRecord;

fn main() {
    let week_range = 104;
    let kde_threshold = 0.4;
    let geo_threshold = 0.75;
    /*****************************LOAD RECORDS*******************************/
    let records = record::read_records_from("../data/data1617.csv"); // Load records
    let tree = clustering::make_record_tree(&records); // Store records in a BTree
    /****************************FILTER RECORDS******************************/
    let curr_time = kernel::current_time(); // Current Time
    let bound_time = kernel::weeks_ago(week_range); // Current Time - x Weeks

    let range_records = tree.range((Included(&bound_time), Included(&curr_time))); // Filter records in range
    /*********************KERNEL DENSITY ESTIMATION***************************/
    let mut kde_out = File::create("../frontend/heatmap.js").expect("Error");
    kde_out
        .write_all(b"function getPoints() {
        return [
            ")
        .expect("Error writing first part");

    let mut geo_records = Vec::new();

    let mut first = true;
    for (_, rec) in range_records {
        let ker_sum = kernel::kernel_sum(rec, &curr_time);
        if ker_sum > kde_threshold {
            let (lat, lon) = rec.get_lat_lon();

            if ker_sum > geo_threshold {
                let mut geo_record = GeoRecord::from_record(rec);
                geo_record.set_kde(ker_sum);
                geo_records.push(geo_record);
            }

            if !first {
                write!(kde_out, ",\n\t\t").expect("Error in line break");
            } else {
                first = false;
            }

            write!(kde_out,
                   "{{ location: new google.maps.LatLng({}, {}), weight: {} }}",
                   lat,
                   lon,
                   (ker_sum - kde_threshold) * (15.0 / (1.0 - kde_threshold)))
                    .expect("Error writing record");
        }
    }

    kde_out
        .write_all(b"    ];
    }

    heatmap = new google.maps.visualization.HeatmapLayer({
        data: getPoints(),
        map: map,
        gradient: gradient
    });")
        .expect("Error writing third part");
    /************************************************************************/
    let mut features = Vec::new();

    for record in &geo_records {
        let mut properties = Map::new();

        properties.insert("description".to_string(),
                          value::Value::String(record.get_description()));
        properties.insert("crime_type".to_string(),
                          value::Value::String(record.get_crime_type()));

        let (lat, lon) = record.get_lat_lon();

        let geojson = Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![lon, lat]))),
            foreign_members: None,
            id: None,
            properties: Some(properties),
        };

        features.push(geojson);
    }

    let feature_collection = GeoJson::FeatureCollection(FeatureCollection {
                                                            bbox: None,
                                                            foreign_members: None,
                                                            features: features,
                                                        });

    let mut geo_out = File::create("../frontend/geo.json").expect("Error creating file");
    write!(geo_out, "{}", feature_collection).expect("Error writing geo info");
    /************************************************************************/
    println!("Hello world!");
}
