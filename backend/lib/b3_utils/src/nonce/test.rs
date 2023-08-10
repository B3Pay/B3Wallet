#[cfg(test)]
mod tests {
    use crate::nonce::Nonce;

    #[test]
    fn test_nonce() {
        let mut nonce = Nonce::new(Some(5));
        assert_eq!(nonce.current(), 5);
        assert_eq!(nonce.next(), 6);
        assert_eq!(nonce.current(), 6);
    }
}
