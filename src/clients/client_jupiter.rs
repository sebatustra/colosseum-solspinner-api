use std::collections::HashMap;
use serde::Deserialize;
use crate::{errors::Result, models::model_position::{Position, PositionWithProfit}};

pub struct JupiterClient;

impl JupiterClient {
    pub async fn get_position_profit(
        position: Position
    ) -> Result<PositionWithProfit> {
        println!("->> {:<12} - get_position_profit", "CLIENT");

        let query_url = format!(
            "https://price.jup.ag/v4/price?ids={}&vsToken={}",
            position.mint_pubkey,
            position.vs_token_symbol
        );
        
        let request = reqwest::get(query_url)
            .await
            .map_err(|e| {
                println!("Jupiter client failed fetching data. Error: {}", e);
                crate::errors::ApiError::JupiterFetchFail
            })?
            .json::<JupiterResponse>()
            .await
            .map_err(|e| {
                println!("Jupiter client failed serializing data. Error: {}", e);
                crate::errors::ApiError::JupiterDeserializationFail
            })?;

        let new_price = request.data.get(&position.mint_pubkey).unwrap().price;

        let (price_change, percentage_change) = calculate_price_change(
            new_price, 
            position.purchase_price
        );

        Ok(PositionWithProfit::new(
            position,
            new_price,
            percentage_change, 
            price_change
        ))
    }
}

fn calculate_price_change(new_price: f64, old_price: f64) -> (f64, f64) {
    let change_in_price = new_price - old_price;
    let percentage_change = if old_price != 0.0 {
        (change_in_price / old_price) * 100.0
    } else {
        0.0 
    };

    (change_in_price, percentage_change)
}

#[derive(Deserialize, Debug)]
struct JupiterResponse {
    data: HashMap<String, TokenData>,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    price: f64
}
