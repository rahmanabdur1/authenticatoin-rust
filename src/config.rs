use mongodb::options::ClientOptions;
use mongodb::Client;

pub async fn create_mongo_client(uri: &str) -> Client {
    let client_options = ClientOptions::parse(uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}
