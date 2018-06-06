extern crate chrono;
extern crate record;

use chrono::{Local, DateTime, NaiveDateTime, Duration};
use record::Record;

pub fn current_time() -> NaiveDateTime {
    let local: DateTime<Local> = Local::now();
    println!("Current time: {}", local);

    local.naive_local()
}

pub fn weeks_ago(weeks: i64) -> NaiveDateTime {
    let local: DateTime<Local> = Local::now();
    let mut past = local.naive_local();
    past -= Duration::weeks(weeks);
    past
}

//Bisquare kernel
pub fn bs_kernel(d: f64, h: f64) -> f64 {
    if d < h {
        (1.0 - (d.powf(2.0) / h.powf(2.0))).powf(2.0)
    } else {
        0.0
    }
}

pub fn kernel_sum(record: &Record, curr: &NaiveDateTime) -> f64 {
    let othr = record.get_date_time();
    let mut sum = 0.0;

    let duration = curr.signed_duration_since(othr);

    let mut d_t = duration.num_minutes() as f64 / 60.0;
    let mut d_w = d_t / 24.0;
    let mut d_s = d_w / 7.0;

    // println!("{}\t{}\t{}", d_t, d_w, d_s);

    d_t = d_t % 24.0;
    d_w = d_w % 7.0;
    d_s = d_s % 52.14;

    if d_t > 12.0 {
        d_t = 24.0 - d_t;
    }

    if d_w > 3.5 {
        d_w = 7.0 - d_w;
    }

    if d_s > 26.07 {
        d_s = 52.14 - d_s;
    }

    // println!("{}\t{}\t{}", d_t, d_w, d_s);

    sum += bs_kernel(d_t, 3.0) * 2.0 / 5.0;        //hours
    sum += bs_kernel(d_w, 2.0) * 2.0 / 5.0;        //days
    sum += bs_kernel(d_s, 6.0) / 5.0;              //weeks

    sum
}