use chrono::prelude::*;

use super::super::super::data::StudentData;

pub fn sort_by_name(vec_to_order: Vec<(String, StudentData)>) -> Vec<(String, StudentData)> {
    let mut data = vec_to_order.clone();
    data.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));

    return data;
}

pub fn sort_by_class(vec_to_order: Vec<(String, StudentData)>) -> Vec<(String, StudentData)> {
    let mut data = vec_to_order.clone();
    data.sort_by(|a, b| a.1.class.to_lowercase().cmp(&b.1.class.to_lowercase()));

    return data;
}

pub fn sort_by_date(vec_to_order: Vec<(String, StudentData)>) -> Vec<(String, StudentData)> {
    let mut data = vec_to_order.clone();

    data.sort_by(|a, b| {
        let date1 = a.1.date.clone();
        let date2 = b.1.date.clone();

        let parsed1 = Local
            .datetime_from_str(&date1, "%Y/%m/%d %H:%M:%S")
            .unwrap();

        let parsed2 = Local
            .datetime_from_str(&date2, "%Y/%m/%d %H:%M:%S")
            .unwrap();

        return parsed1.cmp(&parsed2);
    });
    return data;

    // let date_time = Local::now();

    // let date = date_time.date_naive().format("%Y/%m/%d").to_string();
    // let time = date_time.time().format("%H:%M:%S").to_string();
    // let final_time = format!("{} {}", date, time);

    // log(&final_time);
}
pub fn sort_by_selforder(vec_to_order: Vec<(String, StudentData)>) -> Vec<(String, StudentData)> {
    let mut data = vec_to_order.clone();
    data.sort_by(|a, b| a.1.class_order.cmp(&b.1.class_order));

    return data;
}
