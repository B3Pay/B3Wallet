#[cfg(test)]
mod test {
    use candid::Nat;

    use crate::{
        amount::{error::TokenAmountError, TokenAmount},
        token::ICPToken,
    };

    #[test]
    fn test_add() {
        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 8);

        assert_eq!(amount1 + amount2, Ok(TokenAmount::new(200, 8)));

        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 0);

        assert_eq!(
            amount1 + amount2,
            Err(TokenAmountError::DifferentDecimals(8, 0))
        );

        let amount1 = TokenAmount::new(u128::MAX, 8);
        let amount2 = TokenAmount::new(1, 8);

        assert_eq!(amount1 + amount2, Err(TokenAmountError::Overflow),);
    }

    #[test]
    fn test_sub() {
        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 8);

        assert_eq!(amount1 - amount2, Ok(TokenAmount::new(0, 8)));

        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 0);

        assert_eq!(
            amount1 - amount2,
            Err(TokenAmountError::DifferentDecimals(8, 0))
        );

        let amount1 = TokenAmount::new(0, 8);
        let amount2 = TokenAmount::new(1, 8);

        assert_eq!(amount1 - amount2, Err(TokenAmountError::Underflow),);
    }

    #[test]
    fn test_mul() {
        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 8);

        assert_eq!(amount1 * amount2, Ok(TokenAmount::new(10000, 16)));
        assert_eq!("0.000000000001", TokenAmount::new(10000, 16).to_string());

        let amount1 = TokenAmount::new(100, 0);
        let amount2 = TokenAmount::new(100, 8);

        assert_eq!(amount1 * amount2, Ok(TokenAmount::new(10000, 8)));

        let amount1 = TokenAmount::new(u128::MAX, 8);
        let amount2 = TokenAmount::new(2, 8);

        assert_eq!(amount1 * amount2, Err(TokenAmountError::Overflow),);
    }

    #[test]
    fn test_div() {
        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(100, 8);

        assert_eq!(amount1 / amount2, Ok(TokenAmount::new(1, 8)));

        let amount1 = TokenAmount::new(100, 18);
        let amount2 = TokenAmount::new(100, 18);

        assert_eq!(amount1 / amount2, Ok(TokenAmount::new(1, 18)));

        let amount1 = TokenAmount::new(10000000000, 8);
        let amount2 = TokenAmount::new(100, 0);

        assert_eq!(amount1 / amount2, Ok(TokenAmount::new(1, 8)));

        let amount1 = TokenAmount::new(100, 8);
        let amount2 = TokenAmount::new(0, 8);

        assert_eq!(amount1 / amount2, Err(TokenAmountError::DivisionByZero),);
    }

    #[test]
    fn test_from_tokens() {
        let amount = TokenAmount::from_tokens(ICPToken::from_e8s(1));

        assert_eq!(amount, TokenAmount::new(1, 8));

        let amount = TokenAmount::from_tokens(ICPToken::from_e8s(1000000000));

        assert_eq!(amount, TokenAmount::new(1000000000, 8));
    }

    #[test]
    fn test_to_nat() {
        let amount = TokenAmount::new(100, 8);

        assert_eq!(amount.to_nat(), 100);

        let amount = TokenAmount::new(1000000000000000000, 0);

        assert_eq!(amount.to_nat(), Nat::from(1000000000000000000u64));

        let amount = TokenAmount::new(1010000000000000000, 18);

        assert_eq!(amount.to_nat(), Nat::from(1010000000000000000u64));
    }

    #[test]
    fn test_display() {
        let amount = TokenAmount::new(100, 8);

        assert_eq!(amount.to_string(), "0.000001");

        let amount = TokenAmount::new(1000000000000000000, 0);

        assert_eq!(amount.to_string(), "1000000000000000000");

        let amount = TokenAmount::new(1010000000000000000, 18);

        assert_eq!(amount.to_string(), "1.01");

        let amount = TokenAmount::new(1000000001000000000, 18);

        assert_eq!(amount.to_string(), "1.000000001");

        let amount = TokenAmount::new(100, 1);

        assert_eq!(amount.to_string(), "10");

        let amount = TokenAmount::new(100, 2);

        assert_eq!(amount.to_string(), "1");

        let amount = TokenAmount::new(100, 3);

        assert_eq!(amount.to_string(), "0.1");
    }

    #[test]
    fn test_from_str() {
        let amount = "10.00000000".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(1000000000, 8));

        let amount = "1000000000000000000".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(1000000000000000000, 0));

        let amount = "0.000000000000000001".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(1, 18));

        let amount = "100".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(100, 0));

        let amount = "1.0".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(10, 1));

        let amount = "1.001".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(1001, 3));

        let amount = "1.00000001".parse::<TokenAmount>().unwrap();

        assert_eq!(amount, TokenAmount::new(100000001, 8));
    }
}
