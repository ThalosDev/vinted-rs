use bb8_postgres::tokio_postgres::NoTls;
use lazy_static::lazy_static;
use std::env;
use vinted_rs::{db::DbController, queries::Host, Filter, VintedWrapper};

lazy_static! {
    pub static ref POSTGRES_DB: String =
        std::env::var("POSTGRES_DB").unwrap_or(String::from("vinted-rs"));
    pub static ref POSTGRES_USER: String =
        std::env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    pub static ref POSTGRES_PASSWORD: String =
        std::env::var("POSTGRES_PASSWORD").unwrap_or(String::from("postgres"));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = dotenvy::dotenv(); //Load at runtime

    if args.len() < 2 {
        println!("Please provide the host as a command-line parameter.");
        return;
    }

    let host_arg = args[1].as_str();
    let host: Host = host_arg.into();

    let vinted = VintedWrapper::new_with_host(host);
    println!("Host: {}", vinted.get_host());

    let db_uri = &format!(
        "postgres://{}:{}@localhost/{}?sslmode=disable",
        POSTGRES_USER.clone(),
        POSTGRES_PASSWORD.clone(),
        POSTGRES_DB.clone()
    );

    let db = DbController::new(db_uri, 5, NoTls)
        .await
        .expect("Broken connection to Database, please set it up correctly");

    let adidas = db.get_brand_by_name(&"Adidas").await.unwrap();
    let nike = db.get_brand_by_name(&"Nike").await.unwrap();

    let brands = format!("{},{}", adidas.id, nike.id);

    let filter = Filter::builder()
        .brand_ids(Some(brands))
        .price_from(Some(15.0))
        .price_to(Some(20.0))
        .build();

    let items = vinted
        .get_items(&filter, 10, None, None, None)
        .await
        .unwrap();

    if items.items.is_empty() {
        println!("No items found");
    } else {
        for item in items.items {
            let advanced = vinted
                .get_advanced_item(item.id, None, None, None)
                .await
                .unwrap();
            println!("{}", advanced);
        }
    }
}
