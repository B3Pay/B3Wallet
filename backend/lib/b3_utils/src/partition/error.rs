#[derive(Debug)]
pub enum PartitionManagerError {
    PartitionExists,
    IdAlreadyUsed(String),
    UnableToCreateMemory(String),
}

#[rustfmt::skip]
impl std::fmt::Display for PartitionManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PartitionManagerError::PartitionExists => write!(f, "Partition already exists"),
            PartitionManagerError::IdAlreadyUsed(name) => write!(f, "ID already used for partition {}", name),
            PartitionManagerError::UnableToCreateMemory(err) => write!(f, "Unable to create memory: {:?}", err.to_string())
        }
    }
}
