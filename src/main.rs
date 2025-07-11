mod wallet;

use axum::{
    routing::get,
    // routing:post,
    Router
};

use tokio::net::TcpListener;


#[tokio::main]
async fn main(){

    //1.  Create the axum router
    
    let router_wallet: Router = Router::new().route("/wallet", get(wallet::get_wallet_analytics).post(wallet::post_wallet_analytics)).route("/hexscore",get(wallet::get_wallet_analytics).post(wallet::post_wallet_analytics));

    //2. Define Ip and Port
    let address: &'static str = "0.0.0.0:6570";
    let listener:TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. Axum serve to launch the web server

    axum::serve(listener,router_wallet).await.unwrap();



}


// #[derive(Debug, serde::Serialize)]
// enum WalletBehaviour{
//     SmartMoney,
//     Insider,
//     Retail,
//     Dumper

// }


