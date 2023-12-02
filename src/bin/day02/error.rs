use std::fmt::Display;

#[derive(Debug)]
pub enum GameParseError {
    MissingPrefix,
    MissingSemicolon,
    InvalidGameId,
    MalformedAmountColorPair,
    InvalidColor,
    InvalidAmount,
}

impl Display for GameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameParseError::MissingPrefix => write!(f, r#"missing "Game" prefix "#),
            GameParseError::MissingSemicolon => write!(f, "missing semicolon after game id"),
            GameParseError::InvalidGameId => write!(f, "invalid game id"),
            GameParseError::MalformedAmountColorPair => write!(f, "malformed (amount, color) pair"),
            GameParseError::InvalidColor => write!(f, "invalid cube color"),
            GameParseError::InvalidAmount => write!(f, "invalid cube amount"),
        }
    }
}

impl std::error::Error for GameParseError {}
