#[cfg(test)]
mod tests {
    use crate::token::ICPToken;

    #[test]
    fn test_add() {
        let token1 = ICPToken::from_e8s(100);
        let token2 = ICPToken::from_e8s(200);
        let result = token1 + token2;
        assert_eq!(result.e8s(), 300);
    }

    #[test]
    fn test_sub() {
        let token1 = ICPToken::from_e8s(200);
        let token2 = ICPToken::from_e8s(100);
        let result = token1 - token2;
        assert_eq!(result.e8s(), 100);
    }

    #[test]
    fn test_add_assign() {
        let mut token1 = ICPToken::from_e8s(100);
        let token2 = ICPToken::from_e8s(200);
        token1 += token2;
        assert_eq!(token1.e8s(), 300);
    }

    #[test]
    fn test_sub_assign() {
        let mut token1 = ICPToken::from_e8s(200);
        let token2 = ICPToken::from_e8s(100);
        token1 -= token2;
        assert_eq!(token1.e8s(), 100);
    }

    #[test]
    fn test_is_zero() {
        let token = ICPToken::from_e8s(0);
        assert!(token.is_zero());
    }

    #[test]
    fn test_amount() {
        let token = ICPToken::from_e8s(100_000_000);
        assert_eq!(token.amount(), 1);
    }

    #[test]
    fn test_display() {
        let token = ICPToken::from_e8s(123_456_789);
        assert_eq!(format!("{}", token), "1.23456789");
    }
}
