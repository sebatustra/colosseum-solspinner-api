use std::fmt;

pub type Result<T> = core::result::Result<T, CronError>;

pub enum CronError {
    BirdeyeClientFail,
    FilteredTokensLengthFail,
    UpdateTokenStatusFail,
}

impl fmt::Display for CronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CronError::BirdeyeClientFail => "Birdeye client failed to fetch data.",
                CronError::FilteredTokensLengthFail => "Filtered tokens length is less than 25.",
                CronError::UpdateTokenStatusFail => "Updating token status failed."
            }
        )
    }
}