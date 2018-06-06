extern crate chrono;
extern crate record;

use std::collections::BTreeMap;
use chrono::{/*Local, DateTime, */NaiveDateTime};
use record::Record;

pub fn make_record_tree(records: &[record::Record]) -> BTreeMap<NaiveDateTime, &Record> {
    let mut tree = BTreeMap::new();

    for record in records {
        tree.insert(record.get_date_time(), record);
    }

    tree
}