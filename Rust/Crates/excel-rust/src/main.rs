#[macro_use]
extern crate simple_excel_writer as excel;

use excel::*;
use rand::{distributions::Alphanumeric, Rng, thread_rng};

struct Product {
    reference: String,
    name: String,
    quantity: u8,
    price: f32,
}

fn get_products(num: u32) -> Vec<Product> {
    let mut products = Vec::new();
    for _i in 0..num {
        products.push(
            Product {
                reference: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect(),
                name: String::from("RUNFLAT Evolution gentor"),
                quantity: thread_rng().gen_range(1..10),
                price: thread_rng().gen_range(115.0..365.0)
            }
        )
    }
    return products;
}

fn generate_report(products: Vec<Product>) {
    let mut wb = excel::Workbook::create("./export.xlsx");
    let mut sheet = wb.create_sheet("Export Juillet 2021");

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(excel::row!["Reference", "Produit", "Prix", "Quantité", "Total"])?;
        sw.append_blank_rows(1);
        let mut total_price = 0.0;
        for product in products {
            let total = format!("{:.2}", product.price * product.quantity as f32);
            let price = format!("{:.2}", product.price);
            sw.append_row(excel::row![
                product.reference,
                product.name,
                price,
                product.quantity.to_string(),
                total.clone()
            ])?;
            total_price = total_price + total.parse::<f32>().unwrap();
        }
        sw.append_blank_rows(1);
        sw.append_row(excel::row!["Total", total_price.to_string()])
    }).expect("write excel error!");
    wb.close().expect("close excel error!");
}

pub fn generate_export() {
    let mut wb = excel::Workbook::create("./export_csv_test.xlsx");
    let mut sheet = wb.create_sheet("Export Juillet 2021");

    sheet.add_column(Column { width: 20.0 });
    wb.write_sheet(&mut sheet, |sheet_writer| {

        let sw = sheet_writer;
        sw.append_row(excel::row!["Adresse", "TOTO"])?;

        let name = String::from("TAATAA");
        let addr = String::from("1 rue de la courge");
        let zip = String::from("56548");
        let city = String::from("Les muraux");
        let country = String::from("France");
        let tel = String::from("654689466654");
        let email = String::from("test@test.test");

        println!("{}", format!("{}\n - {}\n - {}\n - {} - {} - {} - {}",
        name,
        addr,
        zip,
        city,
        country,
        tel,
        email
        ));

        // "
        //             <xml>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //                 <tag>{}<tag>
        //             </xml>
        //         ",

        sw.append_row(excel::row![
            format!("{}\n{}\n{}\n{}\n{}\nTel: {}\nEmail: {}\n",
                name,
                addr,
                zip,
                city,
                country,
                tel,
                email
            ),
            "TOTO"
        ])
    }).expect("write excel error!");
    wb.close().expect("close excel error!");
}

fn main() {
    // let products = get_products(50);
    // generate_report(products);
    generate_export()
}
