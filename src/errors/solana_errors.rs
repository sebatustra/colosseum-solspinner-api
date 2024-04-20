use std::fmt;

pub type Result<T> = core::result::Result<T, SolanaError>;

pub enum SolanaError {
    PubkeyParsingFail,
    GetAccountInfoFail
}

impl fmt::Display for SolanaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolanaError::PubkeyParsingFail => "Error parsing Pubkey from str",
                SolanaError::GetAccountInfoFail => "Error getting account info" 
            }
        )
    }
}