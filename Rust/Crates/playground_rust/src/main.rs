use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use csv::{Error, ReaderBuilder};
use std::str::FromStr;

pub struct ProductFilters {
	pub height: u32,
	pub width: u32,
	pub diameter: u32,
	pub product_load: u32,
	pub speed: Option<String>,
	pub product_type: String,
	pub car_brand: Option<String>,
	pub summer: bool,
	pub winter: bool,
	pub four_seasons: bool,
	pub fuel_comsumption: Option<String>,
	pub wet_grip: Option<String>,
	pub rolling_noise: Option<u32>,
	pub homologation: Option<String>,
	pub run_flat: bool,
	pub rim_protection: bool,
	pub self_repairing: bool,
	pub reinforced: bool,
	pub min_price: f32,
	pub max_price: f32
}

fn build_filter_query(filters: &ProductFilters) -> String {
	let mut query = String::from("SELECT * from product WHERE ");
	query.push_str("height = ? AND ");
	query.push_str("width = ? AND ");
	query.push_str("diameter = ? AND ");
	query.push_str("product_load = ? AND ");
	if let Some(_speed) = &filters.speed {
		query.push_str("TRIM(speed) = ? AND ")
	}
	query.push_str("TRIM(product_type) = TRIM(?) AND ");
	if let Some(_car_brand) = &filters.car_brand {
		query.push_str("TRIM(car_brand) = TRIM(?) AND ");
	}
	query.push_str("summer = ? AND ");
	query.push_str("winter = ? AND ");
	query.push_str("four_seasons = ? AND ");
	if let Some(_fuel_comsumption) = &filters.fuel_comsumption {
		query.push_str("fuel_comsumption = ? AND ");
	}
	if let Some(_wet_grip) = &filters.wet_grip {
		query.push_str("wet_grip = ? AND ");
	}
	if let Some(_rolling_noise) = &filters.rolling_noise {
		query.push_str("rolling_noise = ? AND ");
	}
	if let Some(_homologation) = &filters.homologation {
		query.push_str("homologation = ? AND ");
	}
	query.push_str("run_flat = ? AND ");
	query.push_str("rim_protection = ? AND ");
	query.push_str("self_repairing = ? AND ");
	query.push_str("reinforced = ? AND ");
	query.push_str("price BETWEEN ? AND ?");
	query
}

fn build_filters() -> ProductFilters {
    ProductFilters {
        height: 92,
        width: 62,
        diameter: 654,
        product_load: 645,
        speed: None,
        product_type: String::from("Auto"),
        car_brand: None,
        summer: false,
        winter: true,
        four_seasons: true,
        fuel_comsumption: Some(String::from("A")),
        wet_grip: Some(String::from("E")),
        rolling_noise: None,
        homologation: None,
        run_flat: true,
        rim_protection: false,
        self_repairing: true,
        reinforced: false,
        min_price: 10.0,
        max_price: 600.0
    }
}

#[derive(Debug, Deserialize)]
struct Record {
	pub Easy4D_Code: String,
	pub EAN_Code: String,
	pub IPC_Code: String,
	pub Designation: String,
	pub Sculpture: String,
	pub Sculpture_web: String,
	pub Price_list: String,
	pub First_category: String,
	pub Category_tyre_code: String,
	pub Category_tyre_name: String,
	pub Brand_code: String,
	pub Brand_name: String,
	pub Family_code: String,
	pub Family_name: String,
	pub Season: String,
	pub Width: String,
	pub Height: String,
	pub diameter: String,
	pub Construction: String,
	pub TL_TT: String,
	pub Load_index: String,
	pub Speed_index: String,
	pub Special_index: String,
	pub Position: String,
	pub Use: String,
	pub Runflat: String,
	pub DOT: String,
	pub Dismounted: String,
	pub Retread: String,
	pub Weight: String,
	pub Fuel_efficiency: String,
	pub Wet_grip: String,
	pub Noise_level: String,
	pub Noise: String,
	pub OE_homologation: String,
	pub Segmen: String,
}

fn extract_csv_data() {
	let mut contents = open_csv_file().unwrap();
	deserialize_csv_data(&mut contents).expect("toto program");
}

fn open_csv_file() -> std::io::Result<String> {
	let mut file = File::open("demo_catalogue.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
	// println!("content : {}", contents);
	Ok(contents)
}

fn deserialize_csv_data(contents: &mut String) -> Result<(), csv::Error> {
    // let mut reader = csv::Reader::from_reader(contents.as_bytes());
	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(contents.as_bytes());

	for record in reader.records() {
		let record = record?;
		println!("Season {} width {} height {}",
		&record[14], &record[15], &record[16]);
    }
    Ok(())
}

// fn deserialize_csv_data(contents: &mut String) -> Result<(), csv::Error> {
//     // let mut reader = csv::Reader::from_reader(contents.as_bytes());
// 	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(contents.as_bytes());

// 	for record in reader.deserialize::<Record>() {
// 		let record: Record = record?;
// 		println!("record : {:#?}", record);
// 		// println!("Season {} width {} height {}",
//     }
//     Ok(())
// }

fn main() {
    // let filters = build_filters();
    // let query = build_filter_query(&filters);
    // println!("query = {}", query);
	// extract_csv_data();
	// let test = String::from("95,3").replace(",", ".");
	// let number = f64::from_str(&test).unwrap();
	// println!("number = {}", number);

	// let mut s = String::from("(?, ?, ?, ?, ?),");
	// s.truncate(s.len() - 2);
	// println!("s = {}", s);

	let mut v : Vec<String> = Vec::new();

	v.push(String::from("a"));
	v.push(String::from("b"));
	v.push(String::from("c"));
	v.push(String::from("d"));
	v.push(String::from("e"));
	v.push(String::from("f"));
	v.push(String::from("g"));
	v.push(String::from("h"));
	v.push(String::from("i"));
	v.push(String::from("j"));
	v.push(String::from("k"));
	v.push(String::from("l"));
	v.push(String::from("m"));
	v.push(String::from("n"));
	v.push(String::from("o"));
	v.push(String::from("p"));
	v.push(String::from("q"));
	v.push(String::from("r"));
	v.push(String::from("s"));
	v.push(String::from("t"));
	v.push(String::from("u"));
	v.push(String::from("v"));
	v.push(String::from("w"));
	v.push(String::from("x"));
	v.push(String::from("y"));
	v.push(String::from("z"));

	// println!("v = {:#?}", v);

	let mut sp = v.split_off(5);

	while sp.len() > 0 {
		println!("v = {:#?}", v);
		v = sp;
		sp = v.split_off(5);
	}
	// v.append(&mut sp);
	// println!("v = {:#?}", v);
}
