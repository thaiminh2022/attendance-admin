use std::collections::HashMap;
use std::vec;

use super::super::bridge::*;
use super::super::data::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod admin_find;
mod admin_sort;
use admin_find::*;
use admin_sort::*;

pub struct AdminPage {
    // This will be the array for displaying
    student_data_display: Vec<(String, StudentData)>,
    // This is stored a local data and only change when the database is re-fetch
    student_data_local: Vec<(String, StudentData)>,

    find_box: String,
    sort_type: SortType,

    case_sensitive: bool,
    soft_search: bool,
}

pub enum Msg {
    GetDataAllAsync(SortType, bool),

    SaveDataLocal,
    GetDataLocal,

    SortByName,
    SortByDate,
    SortByClass,
    SortBySelfOrder,

    ResetFind,
    FlipSearchMatch(usize),

    FindByName,

    UpdateStudentsData(Vec<(String, StudentData)>, SortType, bool),
    UpdateFindBox(String),
    Refresh,
}

impl Component for AdminPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            student_data_display: vec![],
            student_data_local: vec![],

            sort_type: SortType::None,
            find_box: "".to_string(),

            case_sensitive: false,
            soft_search: true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let map_value = |(index, (_key, value)): (usize, &(String, StudentData))| -> Html {
            html! {
               <tr class = "table-item">
                    <td>{index}</td>
                    <td>{value.name.clone()}</td>
                    <td>{value.class.clone()}</td>
                    <td>{value.class_order}</td>

                    <td>{value.date.clone()}</td>
                </tr>
            }
        };

        let base_classes: [(&str, Html); 2] = [
            ("case-sensitive", html! {<i class="fi fi-bs-a"></i>}),
            ("hard-search", html! {<i class="fi fi-rr-spa"></i>}),
        ];

        let find_options = |(index, (base_class, node)): (usize, &(&str, Html))| -> Html {
            let mut final_class = base_class.to_string().clone();

            if self.soft_search == false && index == 1 {
                final_class.push_str(" find-selected");
            }
            if self.case_sensitive == true && index == 0 {
                final_class.push_str(" find-selected");
            }

            html! {
                <button
                class = {final_class}
                onclick = {link.callback(move |_| Msg::FlipSearchMatch(index))}
                >
                    {{
                        node.clone()
                    }}
                </button>

            }
        };

        let on_find_box_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdateFindBox(input.value()))
        });

        let on_fetch = link.callback(|_| Msg::GetDataAllAsync(SortType::Name, true));
        let on_reset_find = link.callback(|_| Msg::ResetFind);

        let on_order_by_name = link.callback(|_| Msg::SortByName);
        let on_order_by_class = link.callback(|_| Msg::SortByClass);
        let on_order_by_date = link.callback(|_| Msg::SortByDate);
        let on_order_by_selforder = link.callback(|_| Msg::SortBySelfOrder);

        let on_find_by_name = link.callback(|_| Msg::FindByName);

        html! {
            <section class="ad">
                <header class="header">
                    <h1>{"Admin Page"}</h1>
                    <strong>{"Use this to moditor your room"}</strong>
                </header>

                <main class = "main">
                    <div class="keys">

                        <div class="find-box">
                            <input type="text" placeholder = "What do you need to find"
                            onchange = {on_find_box_change}
                            value = {self.find_box.clone()}
                            />

                            <div class="find-custom-btn">
                                <button class="reset-btn" onclick = {on_reset_find}>
                                    <i class="fi fi-sr-cross-circle"></i>
                                </button>

                                <div class="search-options">

                                    {
                                        for base_classes.iter().enumerate().map(find_options)
                                    }

                                </div>
                            </div>

                        </div>

                        <div class="find-btns">
                                <button class ="id-find">{"Find by id"}</button>
                                <button class = "name-find" onclick = {on_find_by_name}>{"Find by name"}</button>
                                <button class = "date-find">{"Find by date"}</button>
                                <button class = "class-find">{"Find by class"}</button>
                        </div>

                    </div>
                    <div class="tables">
                        <table>
                            <tr class ="table-header">
                                <th>{"STT"}</th>
                                <th>
                                    <button onclick = {on_order_by_name} >{"Name 🔍"}</button>
                                </th>
                                <th>
                                    <button onclick = {on_order_by_class} >{"Class 🔍"}</button>
                                </th>
                                <th>
                                    <button onclick = {on_order_by_selforder} >{"Self Order🔍"}</button>
                                </th>
                                <th>
                                    <button onclick = {on_order_by_date} >{"Date 🔍"}</button>
                                </th>
                            </tr>

                            {
                                for self.student_data_display.iter().enumerate().map(map_value)
                            }

                        </table>

                    </div>

                    <div class="custom-btns">
                        <button onclick = {on_fetch}>{"Refresh"}</button>
                        <button>{"Save Data As CSV"}</button>

                    </div>
                </main>

            </section>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::GetDataAllAsync(ot, update_display) => {
                // Force reorder list
                link.send_future(AdminPage::get_all_data(ot, update_display));
                link.send_message(Msg::SaveDataLocal);
            }
            Msg::GetDataLocal => {
                let result = AdminPage::get_data_local();

                match result {
                    Ok(m) => {
                        let final_data = convert_to_vec(m);
                        link.send_message(Msg::UpdateStudentsData(
                            final_data,
                            SortType::Name,
                            true,
                        ));
                    }
                    Err(s) => log(&s),
                };
            }
            Msg::UpdateStudentsData(m, _ot, update_display) => {
                self.student_data_local = sort_by_name(m);

                if update_display {
                    self.student_data_display = self.student_data_local.clone();
                }
            }
            Msg::UpdateFindBox(s) => {
                self.find_box = s;
            }
            Msg::SortByName => {
                let sorted_data = sort_by_name(self.student_data_display.clone());
                self.student_data_display = sorted_data;

                self.sort_type = SortType::Name;
            }
            Msg::SortByClass => {
                self.student_data_display = sort_by_class(self.student_data_display.clone());
                self.sort_type = SortType::Class;
            }
            Msg::SortByDate => {
                self.student_data_display = sort_by_date(self.student_data_display.clone());
                self.sort_type = SortType::Date;
            }
            Msg::SortBySelfOrder => {
                self.student_data_display = sort_by_selforder(self.student_data_display.clone());
                self.sort_type = SortType::SelfOrder;
            }

            Msg::FindByName => {
                let result = find_by_name(
                    self.find_box.clone(),
                    self.student_data_local.clone(),
                    self.case_sensitive,
                    self.soft_search,
                );

                self.student_data_display = result;
            }
            Msg::ResetFind => {
                self.find_box = "".to_string();
                self.student_data_display = self.student_data_local.clone();
            }
            Msg::FlipSearchMatch(i) => {
                if i == 0 {
                    // case sensitive
                    self.case_sensitive = !self.case_sensitive;
                } else {
                    // soft search
                    self.soft_search = !self.soft_search;
                }

                log(&format!(
                    "solf_search: {}, \n case_sensitive: {}",
                    self.soft_search, self.case_sensitive
                ));
            }
            Msg::SaveDataLocal => {
                // Save a local file for faster loading, no internet acess
                let hash_data = convert_to_hasmap(self.student_data_display.clone());
                let data = serde_json::to_string(&hash_data).unwrap();

                set_local_storage("data", data);
            }
            Msg::Refresh => return true,
        }

        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetDataLocal);
            ctx.link()
                .send_message(Msg::GetDataAllAsync(SortType::Name, first_render));
        }
    }
}

impl AdminPage {
    async fn get_all_data(st: SortType, update_display: bool) -> Msg {
        let data = get_data_all().await.as_string().unwrap();
        let take_data: HashMap<String, StudentData> = serde_json::from_str(&data).unwrap();

        let final_data = convert_to_vec(take_data);
        // log(&format!("{:#?}", final_data));

        Msg::UpdateStudentsData(final_data, st, update_display)
    }

    fn get_data_local() -> Result<HashMap<String, StudentData>, String> {
        let get_result = get_local_storage("data");

        let data_raw = match get_result {
            Ok(s) => s,
            Err(v) => {
                return Err(v
                    .as_string()
                    .unwrap_or("No Error Message/ No local file".to_string()))
            }
        };

        if data_raw.trim().is_empty() {
            return Err("NO DATA HERE BITCH".to_string());
        }

        let final_data: HashMap<String, StudentData> = serde_json::from_str(&data_raw).unwrap();

        Ok(final_data)
    }
    fn send_sort_msg(sort_type: SortType) -> Msg {
        match sort_type {
            SortType::None => Msg::Refresh,
            SortType::Name => Msg::SortByName,
            SortType::Date => Msg::SortByDate,
            SortType::Class => Msg::SortByDate,
            SortType::SelfOrder => Msg::Refresh,
        }
    }
}

fn convert_to_vec(data: HashMap<String, StudentData>) -> Vec<(String, StudentData)> {
    let mut final_vec: Vec<(String, StudentData)> = vec![];

    for (key, val) in data {
        final_vec.push((key, val));
    }

    final_vec
}
fn convert_to_hasmap(data: Vec<(String, StudentData)>) -> HashMap<String, StudentData> {
    let mut final_hash = HashMap::new();

    for (key, val) in data {
        final_hash.insert(key, val);
    }

    final_hash
}
