use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum Response {
    Confirm,
    Reject,
}

impl Response {
    pub fn is_confirm(&self) -> bool {
        match self {
            Response::Confirm => true,
            _ => false,
        }
    }

    pub fn is_reject(&self) -> bool {
        match self {
            Response::Reject => true,
            _ => false,
        }
    }
}
