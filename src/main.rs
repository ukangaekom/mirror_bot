use axum::{
    routing::get,
    // routing:post,
    Router
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main(){

    //1.  Create the axum router
    let router01: Router = Router::new().route("/vehicle", get(vehicle_get));

    //2. Define Ip and Port
    let address: &'static str = "0.0.0.0:6570";
    let listener:TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. Axum serve to launch the web server

    axum::serve(listener, router01).await.unwrap();



}


async fn get_wallet_analytics(){

    println!("The following are the wallet analytics");
}