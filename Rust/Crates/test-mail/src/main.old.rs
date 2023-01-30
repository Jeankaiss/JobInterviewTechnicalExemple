use lettre::{
    message::{
        header,
        MultiPart,
        SinglePart
    },
    transport::smtp::authentication::Credentials,
    Message,
    SmtpTransport,
    Transport,
    FileTransport
};

fn send_basic_mail() {
    println!("Build email");
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();
    // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    println!("Set SMTP transport");
    let mailer = SmtpTransport::builder_dangerous(String::from("172.17.0.2")).build();

    // Send the email
    println!("send email");
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    println!("email sent successfuly");
}

fn send_html_mail() {
    println!("build html");
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello from Lettre!</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: center;">
        <h2 style="font-family: Arial, Helvetica, sans-serif;">Hello from Lettre!</h2>
        <h4 style="font-family: Arial, Helvetica, sans-serif;">A mailer library for Rust</h4>
        <h1>THIS MAIL WAS SENT ON MAILDEV BY RUST LETTRE CRATE</h1>
        <h3 style="color:red">THIS TEXT IS RED</h3>
    </div>
</body>
</html>"#;

    // Build the message.
    println!("build email");
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Hello from Lettre!")
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                // .singlepart(
                //     SinglePart::builder()
                //         .header(header::ContentType::TEXT_PLAIN)
                //         .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                // )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .expect("failed to build email");

    // Create our mailer. Please see the other examples for creating SMTP mailers.
    // The path given here must exist on the filesystem.
    println!("build filetransport");
    // let mailer = FileTransport::new("./");
    let mailer = SmtpTransport::builder_dangerous(String::from("172.17.0.2")).build();

    // Store the message when youre ready.
    println!("Send email");
    // mailer.send(&email).expect("failed to deliver message");
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    println!("email sent successfully");
}

fn sendinblue_basic_mail() {

    // let apikey = String::from("api-key:xkeysib-TOTOKEY");

    // let header_name = header::HeaderName::new_from_ascii(String::from("api-key")).unwrap();
    // let header = header::Headers::new()
        // .insert_raw(header_name, String::from("xkeysib-TOTOKEY"));

    println!("Build email");
    let mut email = Message::builder()
        .from("TOTO TATA <TOTO.TATA@gmail.com>".parse().unwrap())
        // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("TITI TUTU <TITI.TUTU@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();
    // email.headers().append_raw(header_name, apikey);
    // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

   // Create our mailer. Please see the other examples for creating SMTP mailers.
    // The path given here must exist on the filesystem.
    println!("build mailer");
    let creds = Credentials::new("TETE.TYTY@gmail.com".to_string(), "pDSDFQSDFQSDFQSDFQ".to_string());
    // let mailer = FileTransport::new("./");
    // let mailer = SmtpTransport::builder_dangerous(String::from("172.17.0.2")).build();
    let mailer = SmtpTransport::relay("smtp-relay.sendinblue.com")
        .unwrap()
        .port(587)
        .credentials(creds)
        .build();

    // Send the email
    println!("send email");
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    println!("email sent successfuly");
}

fn main() {
    // send_basic_mail();
    // send_html_mail();
    sendinblue_basic_mail();
}
