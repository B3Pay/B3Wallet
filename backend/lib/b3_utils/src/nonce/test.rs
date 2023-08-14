#[cfg(test)]
mod tests {
    use crate::nonce::Nonce;

    #[test]
    fn test_nonce() {
        let mut nonce = Nonce::new(Some(5));
        assert_eq!(nonce.current(), Nonce(5));
        assert_eq!(nonce.next(), Nonce(6));
        assert_eq!(nonce.current(), Nonce(6));
    }
}
