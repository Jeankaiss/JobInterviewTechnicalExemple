extern crate printpdf;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use image::png::PngDecoder;
use std::io::Cursor;
// use std::path::Path;
// use image::io::Reader as ImageReader;
use rand::{distributions::Alphanumeric, Rng, thread_rng};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use uuid::Uuid;

pub struct ProductOrder {
	pub uuid: String,
	pub user: String,
	pub reference: String,
	pub total: f32,
	pub delivery_address: Address,
	pub billing_address: Address,
	pub order_date: NaiveDateTime,
	pub po_state: String,
	pub shipping_code: String,
	pub is_read: bool,
	pub has_easy4d: bool,
	pub easy4d_order_code: String,
	pub product_detail: Vec<ProductOrderDetail>,
}

pub struct ProductOrderDetail {
	pub uuid: String,
	pub ean: String,
	pub designation: String,
	pub price: f32,
	pub quantity: u32,
	pub is_easy4d: bool,
	pub easy4d_code: String
}

pub struct Address {
    pub street: String,
    pub zip: String,
    pub city: String,
    pub country: String,
}

fn get_product_order(num : u32) -> ProductOrder {
    let mut product_order = ProductOrder {
        uuid: Uuid::new_v4().to_string(),
        user: Uuid::new_v4().to_string(),
        reference: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect(),
        total: 0.0,
        delivery_address: Address {
            street: String::from("36 rue de la courge molle"),
            zip: String::from("45861"),
            city: String::from("Carcassone de la haute mote cendrée"),
            country: String::from("France"),
        },
        billing_address: Address {
            street: String::from("36 rue de la courge molle"),
            zip: String::from("45861"),
            city: String::from("Carcassone de la haute mote cendrée"),
            country: String::from("France"),
        },
        order_date: NaiveDateTime::new(
            NaiveDate::from_ymd(2015, 6, 3),
            NaiveTime::from_hms_milli(12, 34, 56, 789)
        ),
        po_state: String::from("Paid"),
        shipping_code: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect(),
        is_read: true,
        has_easy4d: true,
        easy4d_order_code: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect(),
        product_detail: Vec::new(),
    };

    for _ in 0..num {
        product_order.product_detail.push(ProductOrderDetail {
            uuid: Uuid::new_v4().to_string(),
            ean: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(15)
                .map(char::from)
                .collect(),
            designation: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(40)
            .map(char::from)
            .collect(),
            price: thread_rng().gen_range(115.0..365.0),
            quantity: thread_rng().gen_range(1..10),
            is_easy4d: true,
            easy4d_code: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(15)
                .map(char::from)
                .collect(),
        });
    }

    let mut total = 0.0;
    for product in &product_order.product_detail {
        total += product.price * product.quantity as f32;
    }
    product_order.total = total;
    product_order
}

fn get_logo() -> Image {
    let image_bytes = include_bytes!("../logo-pneu.png");
    let mut reader = Cursor::new(image_bytes.as_ref());

    let decoder = PngDecoder::new(&mut reader).unwrap();
    let image = Image::try_from(decoder).unwrap();
    
    return image
}

fn add_addresses(
    po: &ProductOrder,
    da: &Address,
    ba: &Address,
    cl: &PdfLayerReference,
    font: &IndirectFontRef,
    fb: &IndirectFontRef) {

    cl.use_text("Adresse de Facturation", 14.0, Mm(10.0), Mm(250.0), &fb);
    cl.use_text(&po.user, 12.0, Mm(10.0), Mm(240.0), &font); // utiliser le last name et first name
    cl.use_text(&ba.street, 12.0, Mm(10.0), Mm(235.0), &font);
    cl.use_text(format!("{}, {}", &ba.zip, &ba.city), 12.0, Mm(10.0), Mm(230.0), &font);
    cl.use_text(&ba.country, 12.0, Mm(10.0), Mm(225.0), &font);
    cl.use_text(format!("Tel: 0165465464"), 12.0, Mm(10.0), Mm(220.0), &font);
    cl.use_text(format!("Email: test@test.test"), 12.0, Mm(10.0), Mm(215.0), &font);

    // FAIRE UN GET USER BY UUID SI ADDRESSE DE STATION DE MONTAGE
    cl.use_text("Adresse de Livraison", 14.0, Mm(110.0), Mm(250.0), &fb);
    cl.use_text(&po.user, 12.0, Mm(110.0), Mm(240.0), &font); // utiliser le last name et first name ou le nom de la station de montage
    cl.use_text(&da.street, 12.0, Mm(110.0), Mm(235.0), &font);
    cl.use_text(format!("{}, {}", &da.zip, &da.city), 12.0, Mm(110.0), Mm(230.0), &font);
    cl.use_text(&da.country, 12.0, Mm(110.0), Mm(225.0), &font);
    cl.use_text(format!("Tel: 0165465464"), 12.0, Mm(110.0), Mm(220.0), &font);
    cl.use_text(format!("Email: test@test.test"), 12.0, Mm(110.0), Mm(215.0), &font);
}

fn add_invoice_general_datas(po: &ProductOrder, cl: &PdfLayerReference, font: &IndirectFontRef, fb: &IndirectFontRef) {
    let mut today = Utc::today().to_string();
    today.truncate(today.len() - 3);    

    let square = Line { 
        points: vec![
            (Point::new(Mm(10.0), Mm(210.0)), false),
            (Point::new(Mm(200.0), Mm(210.0)), false),
            (Point::new(Mm(200.0), Mm(195.0)), false),
            (Point::new(Mm(10.0), Mm(195.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    cl.add_shape(square);
    cl.use_text("Référence de la commande", 12.0, Mm(18.0), Mm(204.0), &fb);
    cl.use_text(&po.reference, 10.0, Mm(26.0), Mm(199.0), &font);

    cl.use_text("Date de la commande", 12.0, Mm(85.0), Mm(204.0), &fb);
    cl.use_text(&po.order_date.date().to_string(), 10.0, Mm(97.0), Mm(199.0), &font);

    cl.use_text("Compte client", 12.0, Mm(155.0), Mm(204.0), &fb);
    cl.use_text(&po.user, 10.0, Mm(139.0), Mm(199.0), &font); // utiliser le user uuid
}

fn add_invoice_datas(_po: &ProductOrder, cl: &PdfLayerReference, font: &IndirectFontRef, fb: &IndirectFontRef) {
    let mut today = Utc::today().to_string();
    today.truncate(today.len() - 3);

    let square = Line { 
        points: vec![
            (Point::new(Mm(130.0), Mm(287.0)), false),
            (Point::new(Mm(200.0), Mm(287.0)), false),
            (Point::new(Mm(200.0), Mm(267.0)), false),
            (Point::new(Mm(130.0), Mm(267.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    cl.add_shape(square);
    cl.use_text("Facture d'avoir n°:", 10.0, Mm(132.0), Mm(280.0), &fb);
    cl.use_text(format!("FA-A-00001"), 10.0, Mm(165.0), Mm(280.0), &font);

    cl.use_text("Date de création de l'avoir: ", 10.0, Mm(132.0), Mm(270.0), &fb);
    cl.use_text(today, 10.0, Mm(175.0), Mm(270.0), &font);
}

fn add_main_carter(cl: &PdfLayerReference, fb: &IndirectFontRef) {
    let square = Line { 
        points: vec![
            (Point::new(Mm(10.0), Mm(190.0)), false),
            (Point::new(Mm(200.0), Mm(190.0)), false),
            (Point::new(Mm(200.0), Mm(180.0)), false),
            (Point::new(Mm(10.0), Mm(180.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(square);

    let square = Line { 
        points: vec![
            (Point::new(Mm(10.0), Mm(180.0)), false),
            (Point::new(Mm(200.0), Mm(180.0)), false),
            (Point::new(Mm(200.0), Mm(60.0)), false),
            (Point::new(Mm(10.0), Mm(60.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(square);

    let line = Line { 
        points: vec![
            (Point::new(Mm(105.0), Mm(180.0)), false),
            (Point::new(Mm(105.0), Mm(60.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(130.0), Mm(180.0)), false),
            (Point::new(Mm(130.0), Mm(60.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(157.0), Mm(180.0)), false),
            (Point::new(Mm(157.0), Mm(60.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(175.0), Mm(180.0)), false),
            (Point::new(Mm(175.0), Mm(60.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    cl.use_text("Produits", 9.0, Mm(15.0), Mm(184.0), &fb);
    cl.use_text("PU (HT)", 9.0, Mm(112.0), Mm(184.0), &fb);
    cl.use_text("Promotion", 9.0, Mm(135.0), Mm(184.0), &fb);
    cl.use_text("Quantité", 9.0, Mm(160.0), Mm(184.0), &fb);
    cl.use_text("Total (HT)", 9.0, Mm(180.0), Mm(184.0), &fb);
}

fn add_total_carter(cl: &PdfLayerReference, fb: &IndirectFontRef) {
    let square = Line { 
        points: vec![
            (Point::new(Mm(150.0), Mm(55.0)), false),
            (Point::new(Mm(200.0), Mm(55.0)), false),
            (Point::new(Mm(200.0), Mm(40.0)), false),
            (Point::new(Mm(150.0), Mm(40.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    cl.add_shape(square);

    let line_1 = Line { 
        points: vec![
            (Point::new(Mm(150.0), Mm(50.0)), false),
            (Point::new(Mm(200.0), Mm(50.0)), false),
            
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    let line_2 = Line { 
        points: vec![
            (Point::new(Mm(150.0), Mm(45.0)), false),
            (Point::new(Mm(200.0), Mm(45.0)), false),
            
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    // let line_3 = Line { 
    //     points: vec![
    //         (Point::new(Mm(150.0), Mm(40.0)), false),
    //         (Point::new(Mm(200.0), Mm(40.0)), false),
            
    //     ],
    //     is_closed: true, 
    //     has_fill: false,
    //     has_stroke: true,
    //     is_clipping_path: false,
    // };

    // let line_4 = Line { 
    //     points: vec![
    //         (Point::new(Mm(150.0), Mm(35.0)), false),
    //         (Point::new(Mm(200.0), Mm(35.0)), false),
            
    //     ],
    //     is_closed: true, 
    //     has_fill: false,
    //     has_stroke: true,
    //     is_clipping_path: false,
    // };

    let line_5 = Line { 
        points: vec![
            (Point::new(Mm(175.0), Mm(55.0)), false),
            (Point::new(Mm(175.0), Mm(40.0)), false),
            
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    cl.add_shape(line_1);
    cl.add_shape(line_2);
    // cl.add_shape(line_3);
    // cl.add_shape(line_4);
    cl.add_shape(line_5);

    cl.use_text("Total HT", 9.0, Mm(151.0), Mm(51.0), &fb);
    cl.use_text("TVA", 9.0, Mm(151.0), Mm(46.0), &fb);
    cl.use_text("20%", 9.0, Mm(177.0), Mm(46.0), &fb);
    cl.use_text("Total TTC", 9.0, Mm(151.0), Mm(41.0), &fb);
}

fn add_company_informations(cl: &PdfLayerReference, fb: &IndirectFontRef) {
    let informations_line_1_0 = "SAS Société du pneu - 938 boulevard du pneumatique";
    let informations_line_1_1 = " - CAPITAL DE 1000.00€ - Gironde la haute - SIRET: 000 000 000 00000";
    let informations_line_1_2 = " - TVA N° FR00 000 000 000 - APE: 4511 Z";
    let informations_line_2 = "Facture soumise aux Conditions Générales de vente de la Société du pneu";
    let informations_line_3 = "Aucun escompte pour paiement anticipé";

    cl.use_text(
        format!("{}{}{}",informations_line_1_0, informations_line_1_1, informations_line_1_2),
        7.0,
        Mm(15.0),
        Mm(15.0),
        &fb
    );
    cl.use_text(informations_line_2, 7.0, Mm(60.0), Mm(10.0), &fb);
    cl.use_text(informations_line_3, 7.0, Mm(80.0), Mm(5.0), &fb);
}

fn add_other_carter(cl: &PdfLayerReference) {
    let square = Line { 
        points: vec![
            (Point::new(Mm(10.0), Mm(287.0)), false),
            (Point::new(Mm(200.0), Mm(287.0)), false),
            (Point::new(Mm(200.0), Mm(27.0)), false),
            (Point::new(Mm(10.0), Mm(27.0)), false)
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(square);

    let line = Line { 
        points: vec![
            (Point::new(Mm(105.0), Mm(287.0)), false),
            (Point::new(Mm(105.0), Mm(27.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(130.0), Mm(287.0)), false),
            (Point::new(Mm(130.0), Mm(27.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(157.0), Mm(287.0)), false),
            (Point::new(Mm(157.0), Mm(27.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);

    let line = Line { 
        points: vec![
            (Point::new(Mm(175.0), Mm(287.0)), false),
            (Point::new(Mm(175.0), Mm(27.0)), false),
        ],
        is_closed: true, 
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    cl.add_shape(line);    
}

fn display_products(po: ProductOrder, doc: &PdfDocumentReference, mut cl: PdfLayerReference, font: &IndirectFontRef, fi: &IndirectFontRef) {
    let mut product_y_offset = 174.0;
    let mut line_y_offset = 160.0;
    let mut total_price: f32 = 0.0;
    let mut product_number = 0;
    let mut pm = 6;
    let main_cl = cl.clone();
    for product in po.product_detail {
        if product_number != 0 && product_number % pm == 0 {
            let (page2,_) = doc.add_page(Mm(210.0), Mm(297.0), "Page 2, Layer 1");
            cl = doc.get_page(page2).add_layer("Layer 2");
            product_y_offset = 281.0;
            line_y_offset = 267.0;
            add_other_carter(&cl);
            pm = 13;
            product_number = 0;
        }
        let total = format!("{:.2}", product.price * product.quantity as f32);
        let price = format!("{:.2}", product.price);
        let ean = format!("EAN : {}", product.ean);

        cl.use_text(&product.designation, 10.0, Mm(14.0), Mm(product_y_offset), &font);
        cl.use_text(format!("Conso carb: A / Freinage: A / Bruit: A (68db) - Loi CE 2020/740"), 7.0, Mm(14.0), Mm(product_y_offset - 4.0), &fi);
        cl.use_text(&ean, 10.0, Mm(14.0), Mm(product_y_offset - 10.0), &font);
        cl.use_text(&price, 10.0, Mm(112.0), Mm(product_y_offset - 4.0), &font);
        cl.use_text(format!("- 0.0%"), 10.0, Mm(135.0), Mm(product_y_offset), &font);
        cl.use_text(format!("- 0.0€"), 10.0, Mm(135.0), Mm(product_y_offset - 6.0), &font);
        cl.use_text(&product.quantity.to_string(), 10.0, Mm(165.0), Mm(product_y_offset - 4.0), &font);
        cl.use_text(&total, 10.0, Mm(182.0), Mm(product_y_offset - 4.0), &font);

        let line = Line { 
            points: vec![
                (Point::new(Mm(10.0), Mm(line_y_offset)), false),
                (Point::new(Mm(200.0), Mm(line_y_offset)), false),
                
            ],
            is_closed: true, 
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };
        cl.add_shape(line);
        product_y_offset = product_y_offset - 20.0;
        line_y_offset = line_y_offset - 20.0;
        total_price = total_price + total.parse::<f32>().unwrap();
        product_number = product_number + 1;
    }

    main_cl.use_text(format!("{:.2}€", po.total), 9.0, Mm(177.0), Mm(51.0), &font);
    // main_cl.use_text(format!("5.0€"), 9.0, Mm(177.0), Mm(46.0), &font); // po.shipping_cost
    // main_cl.use_text(format!("50.0€"), 9.0, Mm(177.0), Mm(41.0), &font);
    main_cl.use_text(format!("{:.2}€", po.total + po.total * 0.2), 9.0, Mm(177.0), Mm(41.0), &font);
}

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Modele de facture", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();
    let font_italic = doc.add_builtin_font(BuiltinFont::TimesItalic).unwrap();
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let po = get_product_order(100);

    get_logo().add_to_layer(current_layer.clone(), Some(Mm(0.0)), Some(Mm(260.0)), None, None, None, None);
    add_invoice_datas(&po, &current_layer, &font, &font_bold);
    add_invoice_general_datas(&po, &current_layer, &font, &font_bold);
    add_addresses(&po, &po.delivery_address, &po.billing_address, &current_layer, &font, &font_bold);
    add_main_carter(&current_layer, &font_bold);
    add_total_carter(&current_layer, &font_bold);
    add_company_informations(&current_layer, &font_bold);

    display_products(po, &doc, current_layer, &font_bold, &font_italic);

    doc.save(&mut BufWriter::new(File::create("Modele de facture.pdf").unwrap())).unwrap();
}
