pub mod email;
pub mod sms;
pub mod push;
pub mod inapp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    EMAIL,
    SMS,
    PUSH,
    INAPP,
}

impl From<&str> for Channel {
    fn from(s: &str) -> Self {
        match s {
            "EMAIL" => Self::EMAIL,
            "SMS" => Self::SMS,
            "PUSH" => Self::PUSH,
            _ => Self::INAPP,
        }
    }
}
