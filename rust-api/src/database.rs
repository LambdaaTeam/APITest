use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub async fn init() -> Result<Database, mongodb::error::Error> {
    let mongo_uri =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in environment variables");
    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();

    let db = Client::with_options(client_options)
        .unwrap()
        .database("rust-api");

    match db.run_command(doc! {"ping": 1}, None).await {
        Ok(_) => Ok(db),
        Err(e) => Err(e),
    }
}
