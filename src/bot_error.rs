#[derive(Debug)]
pub enum BotError {
    Error(String),
    ReqwestError(reqwest::Error),
    SerenityError(serenity::prelude::SerenityError),
}

pub type BotResult<T> = Result<T, BotError>;

// impl From<&str> for BotError {
//     fn from(err: &str) -> BotError {
//         BotError::Error(err.to_string())
//     }
// }
//
// impl From<String> for BotError {
//     fn from(err: String) -> BotError {
//         BotError::Error(err)
//     }
// }

impl From<reqwest::Error> for BotError {
    fn from(err: reqwest::Error) -> BotError {
        BotError::ReqwestError(err)
    }
}

impl From<serenity::prelude::SerenityError> for BotError {
    fn from(err: serenity::prelude::SerenityError) -> BotError {
        BotError::SerenityError(err)
    }
}
