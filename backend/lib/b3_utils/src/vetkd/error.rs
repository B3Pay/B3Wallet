#[derive(Debug)]
pub enum VetKDError {
    CallError(String),
}

#[rustfmt::skip]
impl std::fmt::Display for VetKDError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VetKDError::CallError(msg) => write!(f, "CallError: {}", msg),
        }
    }
}
