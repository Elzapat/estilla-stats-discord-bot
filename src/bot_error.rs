#[derive(Debug)]
pub enum BotError {
    Error(String),
    ReqwestError(reqwest::Error),
    SerenityError(serenity::prelude::SerenityError),
    StdError(std::io::Error),
    RonError(ron::Error),
}

pub type BotResult<T> = Result<T, BotError>;

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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

impl From<std::io::Error> for BotError {
    fn from(err: std::io::Error) -> BotError {
        BotError::StdError(err)
    }
}

impl From<ron::Error> for BotError {
    fn from(err: ron::Error) -> BotError {
        BotError::RonError(err)
    }
}
