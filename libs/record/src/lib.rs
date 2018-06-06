extern crate quick_csv;
extern crate rustc_serialize;
extern crate chrono;
extern crate cogset;

use quick_csv::Csv;
use chrono::{NaiveDate, NaiveDateTime};
use cogset::Point;
use std::str::FromStr;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Record {
    values: Vec<String>,
}

impl Record {
    pub fn get_date_time(&self) -> NaiveDateTime {
        // println!("D: {}, T: {}, WD: {}",
        //          self.values[3],
        //          self.values[4],
        //          self.values[16]);
        let date: Vec<&str> = self.values[3].split("/").collect();
        let mut time: Vec<&str> = self.values[4].split(":").collect();

        if time.len() == 1 {
            time.pop();
            time.push("00");
            time.push("00");
            time.push("00");
        }

        let dt = NaiveDate::from_ymd(i32::from_str_radix(date[2], 10).unwrap(),
                                     u32::from_str_radix(date[0], 10).unwrap(),
                                     u32::from_str_radix(date[1], 10).unwrap())
                .and_hms(u32::from_str_radix(time[0], 10).unwrap(),
                         u32::from_str_radix(time[1], 10).unwrap(),
                         u32::from_str_radix(time[2], 10).unwrap());

        dt
    }

    pub fn get_lat_lon(&self) -> (String, String) {
        (self.values[22].clone(), self.values[21].clone())
    }
}

impl Point for Record {
    fn dist(&self, other: &Record) -> f64 {
        let lat1 = f64::from_str(self.values[22].as_ref()).unwrap();
        let lon1 = f64::from_str(self.values[21].as_ref()).unwrap();
        let lat2 = f64::from_str(other.values[22].as_ref()).unwrap();
        let lon2 = f64::from_str(other.values[21].as_ref()).unwrap();

        let mut distance = (lat1 - lat2).powf(2.0);
        distance += (lon1 - lon2).powf(2.0);

        distance.sqrt()
    }
}

pub fn read_records_from(path: &str) -> Vec<Record> {
    let mut vec = Vec::new();
    let rdr = Csv::from_file(path).unwrap().has_header(true);
    for row in rdr.into_iter() {
        match row.unwrap().decode::<Record>() {
            Ok(vals) => {
                vec.push(vals);
            }
            Err(error) => println!("{}", error),
        }
    }

    vec
}

#[derive(Debug)]
pub struct GeoRecord {
    lat: f64,
    lon: f64,
    day_week: String,
    occur_date: String,
    occur_time: String,
    location: String,
    crime_type: String,
    kde: f64,
    // group_cluster: usize,
}

impl GeoRecord {
    pub fn from_record(rec: &Record) -> GeoRecord {
        GeoRecord {
            lon: rec.values[21].parse::<f64>().unwrap(),
            lat: rec.values[22].parse::<f64>().unwrap(),
            location: rec.values[10].clone(),
            day_week: rec.values[16].clone(),
            occur_date: rec.values[3].clone(),
            occur_time: rec.values[4].clone(),
            crime_type: rec.values[18].clone(),
            kde: 0.0
            // group_cluster: 0
        }
    }

    pub fn set_kde(&mut self, kde: f64) {
        self.kde = kde;
    }

    pub fn get_lat_lon(&self) -> (f64, f64) {
        (self.lat, self.lon)
    }

    pub fn get_description(&self) -> String {
        format!("<h3>{}</h3>{}, {} @ {}<br>Relevancia: {:.3}",
                self.location,
                self.day_week,
                self.occur_date,
                self.occur_time,
                self.kde)
    }

    pub fn get_crime_type(&self) -> String {
        self.crime_type.clone()
    }

    // pub fn set_group_cluster(&mut self, cluster: usize) {
    //     self.group_cluster = cluster;
    // }
}

impl Point for GeoRecord {
    fn dist(&self, other: &GeoRecord) -> f64 {
        let mut distance = (self.lat - other.lat).powf(2.0);
        distance += (self.lon - other.lon).powf(2.0);

        let sim_factor = if self.crime_type == other.crime_type {
            0.5
        } else {
            0.0
        };

        distance /= self.kde + sim_factor;

        distance.sqrt()
    }
}