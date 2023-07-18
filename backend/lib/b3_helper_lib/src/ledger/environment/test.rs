#[cfg(test)]
mod tests {

    use crate::Environment;

    #[test]
    fn test_environment_display() {
        let environment_1 = Environment::Development;
        assert_eq!(environment_1.to_string(), "development");

        let environment_2 = Environment::Production;
        assert_eq!(environment_2.to_string(), "production");

        let environment_3 = Environment::Staging;
        assert_eq!(environment_3.to_string(), "staging");
    }

    #[test]
    fn test_environment_identifier() {
        let environment_1 = Environment::Development;
        assert_eq!(environment_1.identifier(), 255);

        let environment_2 = Environment::Production;
        assert_eq!(environment_2.identifier(), 0);

        let environment_3 = Environment::Staging;
        assert_eq!(environment_3.identifier(), 170);
    }

    #[test]
    fn test_environment_from_identifier() {
        let environment_1 = Environment::from_identifier(255);
        assert_eq!(environment_1, Environment::Development);

        let environment_2 = Environment::from_identifier(0);
        assert_eq!(environment_2, Environment::Production);

        let environment_3 = Environment::from_identifier(170);
        assert_eq!(environment_3, Environment::Staging);
    }
}
