use actix_web::{App, HttpResponse, HttpServer, Responder, error::ErrorInternalServerError, get, post, web, route};
use serde::{Serialize, Deserialize};

use std::io::Write;

use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello World</h1><br/><h3>How are you ?</h3>")
}

#[get("/toto")]
async fn hello_toto() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello Toto</h1>")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>This is the manual hello!</h1>")
}

#[get("/error")]
async fn test_error() -> impl Responder {
    let test_error = ErrorInternalServerError(String::from("23000"));
    println!("test error : {}", test_error);
    if test_error.to_string().eq("23000") == true {
        println!("test worked");
    }
    HttpResponse::Ok().body("test d'erreur")
}

#[derive(Debug, Serialize, Deserialize)]
enum AddrType {
    Delivery,
    Billing,
    AssemblyStation,
}

#[get("/testenum")]
async fn test_enum() -> impl Responder {
    #[derive(Debug, Serialize, Deserialize)]
    struct Address {
        pub name: String,
        pub addrtype: AddrType,
    }
    let addr = Address {name: String::from("1 rue de la courge"),addrtype: AddrType::Delivery};
    HttpResponse::Ok().json(addr)
}

#[get("/soleil")]
async fn get_image() -> impl Responder {
    NamedFile::open("./img/index.jpeg")
}

#[route("/sendimage", method = "POST", method = "OPTIONS")]
async fn send_image(mut payload: Multipart) -> impl Responder {
   println!("HELLO UPLOAD");
   while let Ok(Some(mut field)) = payload.try_next().await {
       let content_type = field.content_disposition().unwrap();
       let extention = content_type.get_filename_ext().unwrap();
       println!("FILE EXTENTION = {}", extention);
       let filename = content_type.get_filename().unwrap();
       let filepath = format!("./img/{}", sanitize_filename::sanitize(&filename));

       let mut f = std::fs::File::create(filepath).unwrap();
       while let Some(chunk) = field.next().await {
           let data = chunk.unwrap();
           f = f.write_all(&data).map(|_| f).unwrap();
       }
   }
   HttpResponse::Ok().body("File uploaded successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .service(hello_toto)
            .service(test_error)
            .route("/manual", web::get().to(manual_hello))
            .service(test_enum)
            .service(get_image)
            .service(actix_files::Files::new("/static", "./img").show_files_listing())
            .service(send_image)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
