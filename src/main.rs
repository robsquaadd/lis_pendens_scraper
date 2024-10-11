use std::io;

fn get_user_info() -> (String, String) {
	let mut start_date = String::new();
	let mut end_date = String::new();
	print!("Welcome to the Lis Pendens Finder!\nThis tool currently only works for Manatee County!\n\n\n");
	println!("Enter the start date (MM-DD-YYYY) for your search: ");
	io::stdin().read_line(&mut start_date).expect("Failed to read line!");
	println!("Enter the end date (MM-DD-YYYY) for your search: ");
	io::stdin().read_line(&mut end_date).expect("Failed to read line!");
	return (start_date, end_date);
}

fn format_url((start_date, end_date): &(String, String), page_number: &i32) -> String {	
	let request_url = format!("https://records.manateeclerk.com/OfficialRecords/Search/InstrumentType/17/{}/{}/{}/50", start_date, end_date, page_number);
	return request_url;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let user_info = get_user_info();
	let mut page_number = 1;
	while page_number < 5 {
		let request_url = format_url(&user_info, &page_number);
		let data = reqwest::get(request_url).await?.text().await?;
		print!("{}", data);
		page_number += 1;
	}
	Ok(())
}
