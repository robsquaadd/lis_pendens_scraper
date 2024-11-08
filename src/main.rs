use scraper::{Html, Selector};
use std::io;

fn get_user_info() -> (String, String) {
    let mut start_date = String::new();
    let mut end_date = String::new();
    print!("Welcome to the Lis Pendens Finder!\nThis tool currently only works for Manatee County!\n\n\n");
    println!("Enter the start date (MM-DD-YYYY) for your search: ");
    io::stdin()
        .read_line(&mut start_date)
        .expect("Failed to read line!");
    println!("Enter the end date (MM-DD-YYYY) for your search: ");
    io::stdin()
        .read_line(&mut end_date)
        .expect("Failed to read line!");
    (start_date, end_date)
}

fn format_url_manatee_county(
    (start_date, end_date): &(String, String),
    page_number: &i32,
) -> String {
    let request_url = format!(
        "https://records.manateeclerk.com/OfficialRecords/Search/InstrumentType/17/{}/{}/{}/50",
        start_date, end_date, page_number
    );
    request_url
}

fn format_html_manatee_county(table_info: Vec<String>) {
    for row in table_info {
        let row_info_parsed = Html::parse_fragment(&table_info);
        let table_info_selector = Selector::parse("td").unwrap();
        for column in row_info_parsed.select(&table_info_selector) {
            //TODO: add the plantiffs, defendents, link to the lis pendens document, and the property description to
            //a database of some sort.
        }
    }
}

async fn get_manatee_county_lis_pendens_data(
    user_info: &(String, String),
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut page_number = 1;
    let mut page_max = 1;
    let mut row_data: Vec<String> = Vec::new();
    let mut pages: Vec<String> = Vec::new();
    loop {
        if page_number > page_max {
            break;
        }
        let request_url = format_url_manatee_county(&user_info, &page_number);
        let page_data = reqwest::get(request_url).await?.text().await?;
        let page_html = Html::parse_document(&page_data);
        let rows = Selector::parse("tr.data-row").unwrap();
        for row in page_html.select(&rows) {
            row_data.push(row.html());
            println!("{}", row.html());
            println!("Row Finsished");
        }
        if page_number == 1 {
            let pages_selector = Selector::parse("ul.pagination").unwrap();
            for pagination_element in page_html.select(&pages_selector) {
                let pagination_html = Html::parse_fragment(&pagination_element.html());
                let individual_page_selector = Selector::parse("a").unwrap();
                for page in pagination_html.select(&individual_page_selector) {
                    pages.push(page.inner_html())
                }
            }
            page_max = pages[pages.len() - 2].parse().unwrap();
        }
        page_number += 1;
    }
    Ok(row_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_info = get_user_info();
    let manatee_county_table_info = get_manatee_county_lis_pendens_data(&user_info).await?;

    //TODO: create a function that formats the results for manatee county
    //TODO: create a function that sens the results as an email.
    Ok(())
}
