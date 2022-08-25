use super::super::super::data::StudentData;
use itertools::Itertools;

// Since id is unique for each person, case sensitive or soft search will not effect this search type
pub fn find_by_id(
    id: String,
    vec_to_find: Vec<(String, StudentData)>,
) -> Vec<(String, StudentData)> {
    let find_result = vec_to_find
        .into_iter()
        .find(|(sid, _data)| sid.clone() == id);

    match find_result {
        Some(s) => vec![s],
        None => vec![],
    }
}

pub fn find_by_name(
    name: String,
    vec_to_find: Vec<(String, StudentData)>,
    case_sensitive: bool,
    soft_search: bool,
) -> Vec<(String, StudentData)> {
    let find_result = vec_to_find
        .into_iter()
        .filter(|(_sid, data)| {
            let mut data_name = data.name.clone();
            let mut search_name = name.clone();

            if case_sensitive == false {
                data_name.make_ascii_uppercase();
                search_name.make_ascii_uppercase();
            }
            if soft_search {
                return data_name.contains(&search_name);
            }

            data_name == search_name
        })
        .collect_vec();

    return find_result;
}

pub fn find_by_class(
    class: String,
    vec_to_find: Vec<(String, StudentData)>,
    case_sensitive: bool,
    soft_search: bool,
) -> Vec<(String, StudentData)> {
    let find_result = vec_to_find
        .into_iter()
        .filter(|(_sid, data)| {
            let mut data_class = data.class.clone();
            let mut search_class = class.clone();

            if case_sensitive == false {
                data_class.make_ascii_uppercase();
                search_class.make_ascii_uppercase();
            }
            if soft_search {
                return data_class.contains(&search_class);
            }

            data_class == search_class
        })
        .collect_vec();

    return find_result;
}

// This is really a tidious way to find, may remove later
pub fn find_by_date(
    date: String,
    vec_to_find: Vec<(String, StudentData)>,
    case_sensitive: bool,
    soft_search: bool,
) -> Vec<(String, StudentData)> {
    let find_result = vec_to_find
        .into_iter()
        .filter(|(_sid, data)| {
            let mut data_date = data.date.clone();
            let mut search_date = date.clone();

            if case_sensitive == false {
                data_date.make_ascii_uppercase();
                search_date.make_ascii_uppercase();
            }
            if soft_search {
                return data_date.contains(&search_date);
            }

            data_date == search_date
        })
        .collect_vec();

    return find_result;
}

// I may use this function later
fn _empty_data(data: String) -> bool {
    return data.trim().is_empty();
}
