use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;
use uuid::Uuid;
use rand::{distributions::Alphanumeric, Rng, thread_rng};
use std::str::FromStr;

pub struct Tire {
	pub uuid: String,                               // =>  x
	pub ean: String,                                // =>  1
	pub reference: String,                          // =>  x
	pub height: u32,                                // =>  16
	pub width: u32,                                 // =>  15
	pub diameter: u32,                              // =>  17
	pub tr_load: u32,                               // =>  20
	pub tr_segment: String,                         // =>  35
	pub tr_speed: String,                           // =>  21
	pub tr_type: String,                            // =>  9
	pub tr_brand: String,                           // =>  11
	pub summer: bool,                               // =>  14
	pub winter: bool,                               // =>  14
	pub four_seasons: bool,                         // =>  14
	pub tr_fuel_consumption: String,                // =>  30
	pub tr_wet_grip: String,                        // =>  31
	pub rolling_noise: u32,                         // =>  33
	pub tr_homologation: String,                    // =>  34
	pub run_flat: bool,                             // =>  25
	pub rim_protection: bool,                       // =>           // ENLEVER
	pub self_repairing: bool,                       // =>           // ENLEVER
	pub reinforced: bool,                           // =>  (special index) 22   // ENLEVER
	pub price: Option<f32>,                         // =>  None
	pub promotion: Option<u32>,                     // =>  None
	pub stock: Option<u32>,                         // =>  None
	pub active: bool,                               // =>  true
	pub is_easy4d: bool,                            // =>  true
    pub easy4d_code: Option<String>,                // =>  0
    pub easy4d_designation: Option<String>,         // => 3
}


// fn extract_csv_data() {
// 	let mut contents = open_csv_file().unwrap();
//     // println!("content: {}", contents);
// 	deserialize_csv_data(&mut contents).expect("toto program");
// }

// fn open_csv_file() -> std::io::Result<String> {
// 	let mut file = File::open("demo_catalogue.csv")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
// 	// println!("content : {}", contents);
// 	Ok(contents)
// }

// fn deserialize_csv_data(contents: &mut String) -> Result<Vec<Tire>, csv::Error> {
// 	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(contents.as_bytes());
// 	let tires = Vec::new();

// 	for record in reader.records() {
// 		let record = record?;
// 		tires.push(Tire{
// 			uuid: Uuid::new_v4().to_string(),
// 			ean: rand::thread_rng()
// 				.sample_iter(&Alphanumeric)
// 				.take(15)
// 				.map(char::from)
// 				.collect(),
// 			reference: rand::thread_rng()
// 				.sample_iter(&Alphanumeric)
// 				.take(15)
// 				.map(char::from)
// 				.collect(),
// 			height: match record.get(16) {
// 				Some(value) => u32::from_str(value).unwrap(),
// 				None => 0,
// 			},
// 			width: match record.get(15) {
// 				Some(value) => u32::from_str(value).unwrap(),
// 				None => 0,
// 			},
// 			diameter: match record.get(17) {
// 				Some(value) => u32::from_str(value).unwrap(),
// 				None => 0,
// 			},
// 			tr_load: match record.get(20) {
// 				Some(value) => u32::from_str(value).unwrap(),
// 				None => 0,
// 			},
// 			tr_segment: match record.get(35) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			tr_speed: match record.get(21) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			tr_type: match record.get(9) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			tr_brand: match record.get(11) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			// summer: record[20].clone(),
// 			// winter: record[20].clone(),
// 			// four_season: record[20].clone(),
// 			tr_fuel_consumption: match record.get(30) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			tr_wet_grip: match record.get(31) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			rolling_noise: match record.get(33) {
// 				Some(value) => u32::from_str(value).unwrap(),
// 				None => 0,
// 			},
// 			tr_homologation: match record.get(34) {
// 				Some(value) => String::from(value),
// 				None => String::new(),
// 			},
// 			// run_flat: record[20].clone(),
// 			// rim_protection: record[20].clone(),
// 			// self_repairing: record[20].clone(),
// 			// reinforced: record[20].clone(),
// 			price: match record.get(33) {
// 				Some(value) => Some(f32::from_str(value).unwrap()),
// 				None => None,
// 			},
// 			// promotion: record[20].clone(),
// 			// stock: record[20].clone(),
// 			active: true,
// 			is_easy4d: true,
// 			easy4d_code: match record.get(0) {
// 				Some(value) => Some(String::from(value)),
// 				None => None,
// 			},
// 			easy4d_designation: match record.get(3) {
// 				Some(value) => Some(String::from(value)),
// 				None => None,
// 			},
// 		});
// 		// println!("Season {} width {} height {} colone 0 {}",
// 		// &record[14], &record[15], &record[16], &record[0]);
//     }
//     Ok(tires)
// }

fn extract_construction() {
	let mut con: Vec<String> = Vec::new();
	let mut file = File::open("Easy4D_Catalogue.csv").expect("toto program");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("tata program");

	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(contents.as_bytes());

	for record in reader.records() {
		let record = record.unwrap();
		let value = match record.get(18) {
			Some(value) => String::from(value),
			None => String::from(""),
		};
		if value.is_empty() {continue;}
		let res = con.iter().find(|&v| v == &value);
		if let None = res {
			con.push(String::from(value));
		}
	}

	println!("valeurs disponibles : {:#?}", con);
}

fn main() {
	extract_construction();
    // extract_csv_data();
}
