#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use rocket::State;
use rocket::form::Form;
use mongodb::{
    bson::{doc, Bson, Binary, Document},
    Client, Collection
};
use std::sync::Arc;
use std::collections::HashMap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct Item {
    user: String,
    name: String,
    serial: i32,
    image: Vec<u8>,
}

#[derive(Debug, Clone)]
struct AppState {
    collection: Arc<Collection<Document>>,
}

#[get("/")]
async fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Home");
    context.insert("greeting", "Welcome To Hunter Serializer!");
    Template::render("index", &context)
}

// TODO: Refactor create route code. This should be more elegant.
// TODO: Format create route to accept file upload for the image.
#[get("/create")]
async fn create_form() -> Template {
    Template::render("create", HashMap::<String, String>::new())
}

#[derive(FromForm)]
struct NewItem {
    user: String,
    name: String,
    serial: i32,
    image: String, 
}

#[post("/create", data = "<new_item_form>")]
async fn create_item(state: &State<AppState>, new_item_form: Form<NewItem>) -> Template {
    let new_item = new_item_form.into_inner();
    
    // Convert image string to Vec<u8>
    let image_data = STANDARD.decode(new_item.image).unwrap_or_default(); // assuming image is Base64 encoded
    
    let doc = doc! {
        "user": new_item.user,
        "name": new_item.name,
        "serial": new_item.serial,
        "image": Bson::Binary(Binary { subtype: mongodb::bson::spec::BinarySubtype::Generic, bytes: image_data }),
    };

    state.collection.insert_one(doc).await.unwrap();

    let mut context = HashMap::new();
    context.insert("title", "Item Created");
    context.insert("message", "Item has been successfully created!");
    Template::render("create", &context)
}

#[launch]
async fn rocket() -> _ {
    let client = Client::with_uri_str("YOUR_CONNECTION_STRING").await.unwrap();
    let database = client.database("rocketry");
    let collection = database.collection("items");
    
    rocket::build()
        .manage(AppState {
            collection: Arc::new(collection),
        })
        .mount("/", routes![index, create_form, create_item])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
