#[derive(Debug)]
pub struct Tire {
	pub uuid: String,
	pub ean: String,									//	=>	1
	pub designation: String,							//	=>	3
	pub model: Option<String>,							//	=>	4
	pub tire_type: Option<String>,						//	=>	9
	pub brand: Option<String>,							//	=>	11
	pub summer: bool,									//	=>	14
	pub winter: bool,									//	=>	14
	pub four_seasons: bool,								//	=>	14
	pub width: String,									//	=>	15
	pub diameter: String,								//	=>	17
	pub height: Option<String>,							//	=>	16
	pub construction: Option<String>,					//	=>	18
	pub tube_type: Option<String>,						//	=>	19
	pub load_index: Option<String>,						//	=>	20
	pub speed: Option<String>,							//	=>	21
	pub runflat: Option<String>,						//	=>	25
	pub fuel_consumption: Option<String>,				//	=>	30
	pub wet_grip: Option<String>,						//	=>	31
	pub noise: Option<String>,							//	=>	33
	pub homologation: Option<String>,					//	=>	34
	pub segment: Option<String>,						//	=>	35
	pub price: f32,										//	(autre catalogue)
	pub promotion_raw: Option<f32>,						//	(0 par defaut)
	pub percent_promotion: Option<u32>,					//	(0 par defaut)
	pub stock: u32,										//	(autre catalogue)
	pub active: bool,									// true
	pub is_easy4d: bool,								// true
	pub easy4d_code: Option<String>,					//	=> 0
}

#[derive(Debug)]
pub struct Stock {
	pub easy4d_code: String,
	pub best_price: f32,
	pub stock_best_price: u32,
	pub delay_best_price: String,
	pub delivery_date_best_price: String,
	pub price_best_delay: String,
	pub stock_best_delay: String,
	pub delay_best_delay: String,
	pub delivery_date_best_delay: String,
}
