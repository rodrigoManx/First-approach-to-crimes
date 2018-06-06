extern crate record;
extern crate kernel;
extern crate clustering;
extern crate rustc_serialize;
extern crate csv;
extern crate cogset;
extern crate geojson;
extern crate serde_json;

use std::collections::Bound::Included;
use std::io::prelude::*;
use cogset::{Dbscan, BruteScan, Euclid};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use serde_json::map::Map;
use serde_json::value;
use record::GeoRecord;
use std::fs::File;

fn main() {
    let curr_time = kernel::current_time(); // Current Time
    let bound_time = kernel::weeks_ago(52); // Current Time - x Weeks

    let records = record::read_records_from("../../data/data1617.csv"); // Load records
    let tree = clustering::make_record_tree(&records); // Store records in a BTree

    let range_records = tree.range((Included(&bound_time), Included(&curr_time))); // Filter records in range
    /**********************************************************/
    let mut geo_records = Vec::new();

    for (_, rec) in range_records {
        let mut geo_record = GeoRecord::from_record(rec);
        geo_record.set_kde(kernel::kernel_sum(rec, &curr_time));
        geo_records.push(geo_record);
    }
    /************************CLUSTERING***********************/
    let mut dbscan;
    {
        let scanner = BruteScan::new(&geo_records);
        /*let mut*/
        dbscan = Dbscan::new(scanner, 0.003, 10);
    }
    /**********************************************************/
    let clusters = dbscan.by_ref().collect::<Vec<_>>();
    let mut features = Vec::new();

    for (i, cluster) in clusters.iter().enumerate() {
        for elem_idx in cluster {
            let mut properties = Map::new();

            properties.insert("description".to_string(),
                              value::Value::String(geo_records[*elem_idx].get_description()));
            properties.insert("crime_type".to_string(),
                              value::Value::String(geo_records[*elem_idx].get_crime_type()));
            properties.insert("cluster_group".to_string(),
                              value::Value::Number(serde_json::Number::from(i + 1)));

            let (lat, lon) = geo_records[*elem_idx].get_lat_lon();

            let geojson = Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(vec![lon, lat]))),
                foreign_members: None,
                id: None,
                properties: Some(properties),
            };

            features.push(geojson);
        }
    }

    let feature_collection = GeoJson::FeatureCollection(FeatureCollection {
                                                            bbox: None,
                                                            foreign_members: None,
                                                            features: features,
                                                        });

    let mut geo_out = File::create("../../frontend/cluster.json").expect("Error creating file");
    write!(geo_out, "{}", feature_collection).expect("Error writing geo info");

    println!("Clustering finalized!!");
}
