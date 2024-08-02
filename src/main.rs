#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use rocket::State;
use mongodb::{
    bson::{doc, Document},
    Client, Collection
};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct AppState {
    collection: Arc<Collection<Document>>,
}

#[get("/")]
async fn index(state: &State<AppState>) -> Template {
    // Insert document if it doesn't already exist
    let filter = doc! { "message": "Hello, MongoDB!" };
    let insert_doc = doc! { "message": "Hello, MongoDB!" };
    
    let result = state.collection.find_one(filter.clone()).await;
    
    if result.unwrap().is_none() {
        state.collection.insert_one(insert_doc).await.unwrap();
    }

    // Retrieve the document
    let doc = state.collection.find_one(filter).await.unwrap().unwrap();
    let message = doc.get_str("message").unwrap_or("No message found").to_string();

    // Create context for the template
    let mut context = HashMap::new();
    context.insert("title", "Welcome");
    context.insert("greeting", &message);

    Template::render("index", &context)
}

#[launch]
async fn rocket() -> _ {
    // Initialize MongoDB client and collection
    let client = Client::with_uri_str("YOUR_CONNECTION_STRING").await.unwrap();
    let database = client.database("rocketry");
    let collection = database.collection("messages");
    
    rocket::build()
        .manage(AppState {
            collection: Arc::new(collection),
        })
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
