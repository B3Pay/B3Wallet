#[derive(Debug)]
pub enum PairingError {
    InvalidLength,
    InvalidCurve,
}

#[rustfmt::skip]
impl std::fmt::Display for PairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PairingError::InvalidLength => write!(f, "Invalid length"),
            PairingError::InvalidCurve => write!(f, "Invalid curve"),
        }
    }
}
