#[macro_use]
extern crate rocket;

use std::env;
use mongodb::{
	bson::doc,
	Client
};
use rocket::routes;

async fn mongodb_ping() -> mongodb::error::Result<()> {
    // Get Mongodb URI
    let uri = env::var("MONGODB_URI").expect("MONGO_URI is not found.");
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;
    // PING
    client.database("admin").run_command(doc! {"ping": 1}, None).await?;
    Ok(())
}

#[get("/<probe>")]
async fn mongodb_healthz(probe: &str) {
    if probe == "readyz" {
        let result = mongodb_ping().await;
        match result {
            Ok(res) => println!("{:#?}", res),
            Err(error) => println!("nOK: {}", error)
        }
    } else {
        println!("nOK")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/mongodb", routes![mongodb_healthz])
}
