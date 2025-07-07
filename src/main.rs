use axum::{
    routing::get,
    // routing:post,
    Router
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main(){

    //1.  Create the axum router
    let router_wallet: Router = Router::new().route("/wallet", get(get_wallet_analytics).post(post_wallet_analytics));


    //2. Define Ip and Port
    let address: &'static str = "0.0.0.0:6570";
    let listener:TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. Axum serve to launch the web server

    axum::serve(listener, router_wallet).await.unwrap();



}

enum WalletBehaviour{
    SmartMoney,
    Insider,
    Retail,
    Dumper

}

struct wallet_analysis{
    wallet: String,
    total_balance:u64,
    behaviour: WalletBehaviour,
    activity_score: u32,
    token_interactions: u32,
    last_action_time: u64
}


async fn get_wallet_analytics() -> Json<wallet_analysis>{

    println!("The following are the wallet analytics");

    Json::from(
        wallet_analysis{
            wallet: "0x564E82722bB9A4E46f48875c25dE11AAD310883E",
            total_balance: 120,
            behaviour: WalletBehaviour.SmartMoney,
            activity_score: 49,
            token_interactions: 1000,
            last_action_time: 1719878400
        }
    );
}

async fn post_wallet_analytics() -> Json<wallet_analysis>{

    Json::from(
        wallet_analysis{
            wallet: "0x564E82722bB9A4E46f48875c25dE11AAD310883E",
            total_balance: 120,
            behaviour: WalletBehaviour.SmartMoney,
            activity_score: 49,
            token_interactions: 1000,
            last_action_time: 1719878400
        }

    );

}