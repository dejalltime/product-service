use crate::model::{Product};

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{middleware, App, HttpServer};
use actix_web::{web};

use crate::configuration::Settings;
use crate::routes::*;
use mongodb::{Client, Collection};


pub async fn run(mut settings: Settings) -> Result<Server, std::io::Error> {
    
    // let products = fetch_products(&settings);

    let mongo_client = Client::with_uri_str(&settings.mongo_uri).await.unwrap();
    let product_collection = mongo_client
        .database(var("PRODUCT_DB_NAME").unwrap_or("best_buy".to_string()))
        .collection::<Product>(var("PRODUCT_COLLECTION_NAME").unwrap_or("products".to_string()));

    let listener = settings.get_tcp_listener()?;
    let port = listener.local_addr().unwrap().port();
    println!("Listening on http://0.0.0.0:{}", port);

    
    let product_state = web::Data::new(AppState {
        // products: Mutex::new(products.to_vec()),
        product_collection,
        settings: settings,
    });

    let server = HttpServer::new(move || {
        
        let cors = Cors::permissive();

        App::new()
        .wrap(cors)
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
        .app_data(product_state.clone())
        .route("/health", web::get().to(health))
        .route("/health", web::head().to(health))
        .route("/{product_id}", web::get().to(get_product))
        .route("/", web::get().to(get_products))
        .route("/", web::post().to(add_product))
        .route("/", web::put().to(update_product))
        .route("/{product_id}", web::delete().to(delete_product))
        .route("/ai/health", web::get().to(ai_health))
        .route("/ai/health", web::head().to(ai_health))
        .route(
            "/ai/generate/description",
            web::post().to(ai_generate_description),
        )
        .route(
            "/ai/generate/image",
            web::post().to(ai_generate_image),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub struct AppState {
    // pub products: Mutex<Vec<Product>>,
    pub settings: Settings,
    pub product_collection: Collection<Product>, 
}


