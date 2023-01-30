use csv::ReaderBuilder;
use uuid::Uuid;

use crate::{
    Schema::{
		Tire,
		Stock,
	},
    Constants::{
        SUMMER,
        WINTER,
        FOUR_SEASONS
    }
};

pub fn deserialize_tires(content: &mut String, stock: Vec<Stock>) -> Result<Vec<Tire>, csv::Error> {
	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
	let mut tires = Vec::new();

	for record in reader.records() {
		let record = record?;

		let new_ean = String::from(record.get(1).unwrap());
		if new_ean.is_empty() { continue; }

		// check si le pneu est présent dans le stock
		let code = record.get(0).unwrap();
		let stock_element = match stock.iter().find(|&elem| elem.easy4d_code == code) {
			Some(element) => element,
			None => continue,
		};
		tires.push(Tire{
			uuid: Uuid::new_v4().to_string(),
			ean: new_ean,
			designation: String::from(record.get(3).unwrap()),
			model: match record.get(4) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			tire_type: match record.get(9) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			brand: match record.get(11) {
				Some(value) => Some(String::from(value).to_uppercase()),
				_ => None,
			},
			summer: String::from(record.get(20).unwrap()).eq(SUMMER),
			winter: String::from(record.get(20).unwrap()).eq(WINTER),
			four_seasons: String::from(record.get(20).unwrap()).eq(FOUR_SEASONS),
			width: match record.get(15) {
				Some(value) => String::from(value),
				None => continue,
			},
			diameter: match record.get(17) {
				Some(value) => String::from(value),
				None => continue,
			},
			height: match record.get(16) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			construction: match record.get(18) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			tube_type: match record.get(19) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			load_index: match record.get(20) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			speed: match record.get(21) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			runflat: match record.get(25) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			fuel_consumption: match record.get(30) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			wet_grip: match record.get(31) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			noise: match record.get(33) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			homologation: match record.get(34) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			segment: match record.get(35) {
				Some(value) => Some(String::from(value)),
				_ => None,
			},
			price: stock_element.best_price,
			promotion_raw: None,
			percent_promotion: None,
			stock: stock_element.stock_best_price,
			active: true,
			is_easy4d: true,
			easy4d_code: Some(String::from(code)),
		});
    }
    Ok(tires)
}

pub fn deserialize_stock(content: &mut String) -> Result<Vec<Stock>, csv::Error> {
	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
	let mut stock = Vec::new();

	for record in reader.records() {
		let record = record?;
		stock.push(Stock {
			easy4d_code: String::from(record.get(0).unwrap()),
			best_price: String::from(record.get(1).unwrap()).replace(",", ".").parse::<f32>().unwrap(),
			stock_best_price: String::from(record.get(2).unwrap()).parse::<u32>().unwrap(),
			delay_best_price: String::from(record.get(3).unwrap()),
			delivery_date_best_price: String::from(record.get(4).unwrap()),
			price_best_delay: String::from(record.get(5).unwrap()),
			stock_best_delay: String::from(record.get(6).unwrap()),
			delay_best_delay: String::from(record.get(7).unwrap()),
			delivery_date_best_delay: String::from(record.get(8).unwrap()),
		});
	}
	Ok(stock)
}
