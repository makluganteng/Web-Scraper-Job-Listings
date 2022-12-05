use std::path;

use scraper::{Html, Selector};
fn main() {
    let response = reqwest::blocking::get("https://www.imdb.com/chart/top/?ref_=nv_mv_250")
    .unwrap()
    .text()
    .unwrap();

    let document = Html::parse_document(&response);

    let movieSelector = Selector::parse("tbody.lister-list").unwrap();
    let movie_name_selector = Selector::parse("td.titleColumn a").unwrap();

    for element in document.select(&movieSelector) {
        let movie_name_element = element.select(&movie_name_selector).next().expect("Not Found");
        let movie_name = movie_name_element.text().collect::<String>();
        println!("{:?}",movie_name);
    }
}

