use crate::VintedWrapper;
use crate::db::DbController;
use crate::model::filter::{Currency, Filter};
use crate::queries::VintedWrapperError;
use crate::tests::DB_URI;
use bb8_postgres::tokio_postgres::NoTls;
use env_logger;

const POOL_SIZE: u32 = 5;

fn _calculate_color_props(hex_color1: &str) -> (f64, f64, f64) {
    let color1 = _hex_to_rgb(hex_color1);

    let r_prop = color1.0 as f64 / 255.0;
    let g_prop = color1.1 as f64 / 255.0;
    let b_prop = color1.2 as f64 / 255.0;

    (r_prop, g_prop, b_prop)
}

fn _hex_to_rgb(hex_color: &str) -> (u8, u8, u8) {
    let hex = hex_color.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r, g, b)
}

#[tokio::test]
async fn test_get_item_query_text() {
    let vinted = VintedWrapper::new();

    let filter: Filter = Filter::builder()
        .search_text(Some(String::from("shoes")))
        .build();

    match vinted.get_items(&filter, 1, None, None, None).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            assert!(items.items.len() <= 1);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(err) => {
                    log::error!("Unexpected CookiesError: {:?}", err);
                    panic!("CookiesError occurred unexpectedly: {:?}", err);
                }
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_item_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(&DB_URI, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder()
        .brand_ids(Some(brand.id.to_string()))
        .build();

    match vinted.get_items(&filter, 1, None, None, None).await {
        // Limitado el numero de elementos a 1
        Ok(items) => {
            let result = items.items.first();
            if result.is_none() {
                drop(VintedWrapperError::ItemNumberError);
            }
            assert_eq!(result.unwrap().brand_title, brand.title);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_brands() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(&DB_URI, POOL_SIZE, NoTls).await.unwrap();
    let brand = db.get_brand_by_name(&String::from("Adidas")).await.unwrap();

    let filter: Filter = Filter::builder()
        .brand_ids(Some(brand.id.to_string()))
        .build();

    match vinted.get_items(&filter, 10, None, None, None).await {
        Ok(items) => {
            for item in items.items {
                assert_eq!(item.brand_title, brand.title);
            }
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
#[ignore]
async fn test_get_items_catalogs_no_db() {
    let vinted = VintedWrapper::new();
    //Woman elements
    let filter: Filter = Filter::builder()
        .catalog_ids(Some(String::from("1904")))
        .build();
    let substrings = vec![
        "women", "mujer", "femme", "kobiety", "donna", "moterims", "noi", "dames", "zeny", "damen",
        "femei", "mulher", "beauty", "femmes", "dam", "hombre",
    ];

    match vinted.get_items(&filter, 10, None, None, None).await {
        Ok(items) => {
            assert!(items.items.len() <= 10);
            items.items.iter().for_each(|item| {
                let url_item: &str = &item.url;
                let category = url_item.split('/').nth(3).unwrap();
                println!("{:?}", category);
                assert!(
                    substrings.contains(&category),
                    "Category not found {}",
                    category
                );
            });
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_price() {
    let vinted = VintedWrapper::new();
    let min = 50.0;
    let max = 100.0;

    let filter: Filter = Filter::builder()
        .price_from(Some(min))
        .price_to(Some(max))
        .build();

    match vinted.get_items(&filter, 10, None, None, None).await {
        Ok(items) => {
            assert!(items.items.len() <= 10);
            let ok: bool = items.items.iter().all(|item| {
                let price: f32 = item.price.amount.parse().unwrap();
                price <= max && price >= min
            });

            assert!(ok);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_size_no_db() {
    let vinted = VintedWrapper::new();
    let size_id = String::from("1568");
    let size_title = String::from("XS");

    let filter: Filter = Filter::builder().size_ids(Some(size_id)).build();

    match vinted.get_items(&filter, 20, None, None, None).await {
        Ok(items) => {
            assert!(items.items.len() <= 20);
            let ok: bool = items.items.iter().all(|item| item.size_title == size_title);

            assert!(ok);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_size() {
    let vinted = VintedWrapper::new();
    let db: DbController<NoTls> = DbController::new(&DB_URI, POOL_SIZE, NoTls).await.unwrap();
    let size = db
        .get_size_by_title_and_type(
            &String::from("ES"),
            &"XL".to_string(),
            &"Pantalones de hombre".to_string(),
        )
        .await
        .unwrap();

    let filter: Filter = Filter::builder()
        .size_ids(Some(size.id.to_string()))
        .build();

    match vinted.get_items(&filter, 20, None, None, None).await {
        Ok(items) => {
            assert!(items.items.len() <= 20);
            let ok: bool = items
                .items
                .iter()
                .all(|item| item.size_title == size.title_es);

            assert!(ok);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_material() {
    let vinted = VintedWrapper::new();
    let id = 49; // Silk

    let filter: Filter = Filter::builder().material_ids(Some(id.to_string())).build();
    let num: usize = 15;

    match vinted
        .get_items(&filter, num as u32, None, None, None)
        .await
    {
        Ok(items) => {
            assert!(items.items.len() <= num);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_color() {
    let vinted = VintedWrapper::new();
    let id = 7; //Red
    //let hex = "#CC3300"; //Red

    //let props = calculate_color_props(hex);

    let filter: Filter = Filter::builder().color_ids(Some(id.to_string())).build();

    let num: usize = 20;

    match vinted
        .get_items(&filter, num as u32, None, None, None)
        .await
    {
        Ok(items) => {
            assert!(items.items.len() <= num);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_items_by_currency() {
    let vinted = VintedWrapper::new_with_currency(Currency::CZK);

    let filter: Filter = Filter::builder().build();

    let num: usize = 20;

    match vinted
        .get_items(&filter, num as u32, None, None, None)
        .await
    {
        Ok(items) => {
            assert!(items.items.len() <= num);
            let ok: bool = items.items.iter().all(|item| {
                let c: &str = Currency::CZK.into();
                item.price.currency_code == c
            });

            assert!(ok);
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}

#[tokio::test]
async fn test_get_advanced_items() {
    env_logger::builder().is_test(true).init();
    let db = DbController::new(&DB_URI, 5, NoTls).await.unwrap();

    let adidas = db.get_brand_by_name(&"Adidas").await.unwrap();
    let nike = db.get_brand_by_name(&"Nike").await.unwrap();

    let brands = format!("{},{}", adidas.id, nike.id);

    let filter = Filter::builder()
        .brand_ids(Some(brands))
        .price_from(Some(15.0))
        .price_to(Some(20.0))
        .build();

    let vinted = VintedWrapper::new();

    match vinted.get_items(&filter, 10, None, None, None).await {
        Ok(items) => {
            if !items.items.is_empty() {
                for item in items.items {
                    let raw = vinted.get_advanced_item(item.id, None, None, None).await;
                    match raw {
                        Ok(advanced) => {
                            assert_eq!(item.id, advanced.id);
                        }
                        Err(err) => {
                            log::error!("{:#?}", err);
                            match err {
                                VintedWrapperError::ItemNumberError => unreachable!(),
                                VintedWrapperError::ItemError(_, _, _) => (),
                                VintedWrapperError::CookiesError(_) => unreachable!(),
                                VintedWrapperError::SerdeError(_) => unreachable!(),
                                VintedWrapperError::ReqWestError(_) => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            log::error!("{:#?}", err);
            match err {
                VintedWrapperError::ItemNumberError => unreachable!(),
                VintedWrapperError::ItemError(_, _, _) => unreachable!(),
                VintedWrapperError::CookiesError(_) => unreachable!(),
                VintedWrapperError::SerdeError(_) => unreachable!(),
                VintedWrapperError::ReqWestError(_) => unreachable!(),
            }
        }
    };
}
