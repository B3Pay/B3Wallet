#[cfg(test)]
mod tests {
    use crate::http::{
        HttpRequest, HttpsOutcallCost, HTTPS_OUTCALL_BASE_COST, HTTPS_OUTCALL_REQ_COST_PER_BYTE,
        HTTPS_OUTCALL_RESP_COST_PER_BYTE,
    };
    use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpMethod};

    #[test]
    fn test_new_http_request() {
        let request = HttpRequest::new("https://example.com".to_string());
        assert_eq!(request.0.url, "https://example.com");
        assert_eq!(request.0.method, HttpMethod::GET);
        assert!(request.0.headers.is_empty());
        assert!(request.0.body.is_none());
        assert!(request.0.max_response_bytes.is_none());
        assert!(request.0.transform.is_none());
    }

    #[test]
    fn test_http_request_get() {
        let request = HttpRequest::new("https://example.com".to_string()).get(None);
        assert_eq!(request.0.method, HttpMethod::GET);
    }

    #[test]
    fn test_http_request_post() {
        let request =
            HttpRequest::new("https://example.com".to_string()).post("{}", Some(1024 * 1024));
        assert_eq!(request.0.method, HttpMethod::POST);
        assert_eq!(request.0.body.unwrap(), b"{}".to_vec());
        assert_eq!(request.0.max_response_bytes.unwrap(), 1024 * 1024);
    }

    #[test]
    fn test_https_outcall_cost_total() {
        let request = CanisterHttpRequestArgument {
            url: "https://example.com".to_string(),
            headers: vec![],
            method: HttpMethod::GET,
            max_response_bytes: Some(1024 * 1024),
            transform: None,
            body: None,
        };

        let expected_cost = HTTPS_OUTCALL_BASE_COST
            + HttpsOutcallCost::enc_arg_size(&request) * HTTPS_OUTCALL_REQ_COST_PER_BYTE
            + HttpsOutcallCost::max_resp_bytes(&request) * HTTPS_OUTCALL_RESP_COST_PER_BYTE;

        assert_eq!(HttpsOutcallCost::total(&request), expected_cost);
    }

    #[test]
    fn test_default_content_type_for_post() {
        let req = HttpRequest::new("https://example.com".to_string()).post("{}", None);

        let content_type_header = req
            .0
            .headers
            .iter()
            .find(|h| h.name == "Content-Type")
            .expect("Content-Type header not found");

        assert_eq!(content_type_header.value, "application/json");
    }

    #[test]
    fn test_overwrite_content_type() {
        let req = HttpRequest::new("https://example.com".to_string())
            .post("{}", None)
            .content_type("application/xml");

        let content_type_header = req
            .0
            .headers
            .iter()
            .find(|h| h.name == "Content-Type")
            .expect("Content-Type header not found");

        assert_eq!(content_type_header.value, "application/xml");
    }

    #[test]
    fn test_add_query_params() {
        let req = HttpRequest::new("https://example.com".to_string()).add_query_params(vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);

        assert_eq!(req.0.url, "https://example.com?key1=value1&key2=value2");
    }

    #[test]
    fn test_calculate_cycle_cost() {
        let req = HttpRequest::new("https://example.com".to_string()).post("{}", Some(1024));

        let expected_cost = HttpsOutcallCost::total(&req.0);
        let actual_cost = req.calculate_cycle_cost();

        assert_eq!(expected_cost, actual_cost);
    }
}
