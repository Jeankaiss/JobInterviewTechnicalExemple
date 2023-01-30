use dotenv;
use ftp::FtpStream;

use crate::Constants::{
	UNABLE_TO_PARSE_CATALOG,
	UNINITIALIZED_VARIABLE,
	UNABLE_TO_PARSE_STOCK,
};

pub fn get_provider_catalog() -> String {
	let easy4d_url = dotenv::var("URL_EASY4D").expect(UNINITIALIZED_VARIABLE);
	let easy4d_id = dotenv::var("IDENTIFIANT_EASY4D").expect(UNINITIALIZED_VARIABLE);
	let easy4d_pswd = dotenv::var("PASSWORD_EASY4D").expect(UNINITIALIZED_VARIABLE);
	let easy4d_file = dotenv::var("FILE_EASY4D").expect(UNINITIALIZED_VARIABLE);

	let mut ftp_stream = FtpStream::connect((easy4d_url, 21)).unwrap();
	let _ = ftp_stream.login(&easy4d_id, &easy4d_pswd).unwrap();
	let remote_file = ftp_stream.simple_retr(&easy4d_file).unwrap().into_inner();

	String::from_utf8(remote_file).expect(UNABLE_TO_PARSE_CATALOG)
}

pub fn get_provider_stock() -> String {
	let easy4d_url = dotenv::var("URL_EASY4D").expect(UNINITIALIZED_VARIABLE);
	let inet_id = dotenv::var("IDENTIFIANT_INET").expect(UNINITIALIZED_VARIABLE);
	let inet_pswd = dotenv::var("PASSWORD_INET").expect(UNINITIALIZED_VARIABLE);
	let stock_file = dotenv::var("STOCK_FILE").expect(UNINITIALIZED_VARIABLE);

	let mut ftp_stream = FtpStream::connect((easy4d_url, 21)).unwrap();
	let _ = ftp_stream.login(&inet_id, &inet_pswd).unwrap();
	let remote_file = ftp_stream.simple_retr(&stock_file).unwrap().into_inner();

	String::from_utf8(remote_file).expect(UNABLE_TO_PARSE_STOCK)
}
