use axum::{Json,debug_handler};
use reqwest;



#[derive(Debug, serde::Serialize)]
pub struct WalletAnalysis{
    wallet: String,
    total_balance:u64,
    behaviour: String,
    activity_score: u32,
    token_interactions: u32,
    last_action_time: u64
}
// 0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326


// #[tokio::main]
// pub async fn get_transaction_details<T>(address:&str) -> String {

//     let transactions = format!("'https://deep-index.moralis.io/api/v2.2/{}/erc20/transfers?chain=eth&order=DESC'",address);
        
//     let response = reqwest::get(&transactions).await.expect("REASON");
    
//     let market_cap : Json<T> =  response;
    
//     // if market_cap.get("DISPLAY").is_some() {
//     //     let json_response = format!("{}",&market_cap.get("DISPLAY")
//     //     .expect("REASON")
//     //     .get(&address)
//     //     .expect("REASON")
//     //     .get("USD")
//     //     .expect("REASON")
//     //     .get("MKTCAP")
//     //     .unwrap()
//     //     .as_str()
//     //     .unwrap());

//     //     return json_response;
//     // }else{

//     //     return "Error fetching data!".to_string();
//     // }
//     return format!("{:?}",market_cap)

// }





#[debug_handler]
pub async fn get_wallet_analytics() -> Json<WalletAnalysis> {

    println!("The following are the wallet analytics");

    return Json::from(
        WalletAnalysis{
            wallet: "0x564E82722bB9A4E46f48875c25dE11AAD310883E".to_string(),
            total_balance: 120,
            behaviour:"Dumper".to_string(),
            activity_score: 49,
            token_interactions: 1000,
            last_action_time: 1719878400
        }
    );
}


#[debug_handler]
pub async fn post_wallet_analytics() -> Json<WalletAnalysis>{

    return Json::from(
        WalletAnalysis{
            wallet: "0x564E82722bB9A4E46f48875c25dE11AAD310883E".to_string(),
            total_balance: 120,
            behaviour: "Dumper".to_string(),
            activity_score: 49,
            token_interactions: 1000,
            last_action_time: 1719878400
        }

    );

}

#[debug_handler]
pub async fn post_hexscore(){
    
}

#[debug_handler]
pub async fn get_hexscore(){

}