use tokio_cron_scheduler::Job;

use crate::{
    errors::cron_errors::{CronError, Result}, models::model_token::Token, AppState
};

pub struct TokenUpdater;

impl TokenUpdater {
    pub fn init_job(
        job_schedule: &str,
        state: AppState
    ) -> Job {
        Job::new_async(job_schedule, move |_, _| {
            let state_copy = state.clone();
            Box::pin(async move {
                let mut attempts = 0;

                while attempts < 3 {
                    match Self::run_token_updater(
                        state_copy.clone(),
                    ).await {
                        Ok(_) => {
                            println!("->> {:<12} - run_token_updater succeeded", "CRON");
                            break;
                        },
                        Err(e) => {
                            println!("->> {:<12} - run_token_updater failed. Error: {e}", "CRON");
                            attempts += 1;

                            if attempts >= 3 {
                                println!("->> {:<12} - run_token_updater failed 3 times", "CRON");
                                break;
                            }
                        }
                    }
                }
            })
        }).expect("Failed to add job") 
    }
}

impl TokenUpdater {
    pub async fn run_token_updater(
        state: AppState
    ) -> Result<()> {
        println!("->> {:<12} - run_token_updater", "UPDATER");

        let birdeye_client = &state.birdeye_client;

        // get all tokens
        let tokens = Token::get_tokens(state.clone())
            .await.map_err(|_| CronError::UpdateTokenStatusFail)?;

        if tokens.len() == 0 {
            println!("No tokens to update");
            return Err(CronError::UpdateTokenStatusFail)
        }

        // fetch token overview for them
        for token in tokens {
            let token_overview = 
                birdeye_client.get_token_overview(&token.mint_pubkey)
                .await.map_err(|_| CronError::BirdeyeClientFail)?;

            // update the data

            Token::update_token_financial_data(
                &token.mint_pubkey, 
                token_overview.data.price_change_24h_percent, 
                token_overview.data.volume_24h_usd, 
                token_overview.data.decimals, 
                state.clone()
            ).await.map_err(|_| CronError::UpdateTokenStatusFail)?;

        }


        Ok(())
    }
}