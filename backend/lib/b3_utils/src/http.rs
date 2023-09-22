mod test;

mod cost;
pub use cost::*;

use ic_cdk::api::management_canister::http_request::{
    http_request, http_request_with_closure, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    HttpResponse, TransformContext,
};

/// Used to build a request to the Management Canister's `http_request` method.
pub struct HttpRequest(pub CanisterHttpRequestArgument);

impl HttpRequest {
    /// Creates a new request to be built up by having
    pub fn new(url: String) -> Self {
        Self(CanisterHttpRequestArgument {
            url,
            headers: vec![],
            method: HttpMethod::GET,
            max_response_bytes: None,
            transform: None,
            body: None,
        })
    }

    /// A simple wrapper to assign the URL with the `GET` method.
    /// The `max_response_bytes` is set to the `max_response_bytes` argument.
    /// The `max_response_bytes` argument is optional.
    pub fn get(self, max_response_bytes: Option<u64>) -> Self {
        self.method(HttpMethod::GET)
            .max_response_bytes(max_response_bytes)
    }

    /// A simple wrapper to assign the URL with the `HEAD` method.
    /// The `max_response_bytes` is set to the `max_response_bytes` argument.
    /// The `max_response_bytes` argument is optional.
    /// The `HEAD` method is used to retrieve the headers of the response.
    /// The body of the response is empty.
    pub fn head(self, max_response_bytes: Option<u64>) -> Self {
        self.method(HttpMethod::HEAD)
            .max_response_bytes(max_response_bytes)
    }

    /// A simple wrapper to assign the URL with the `POST` method.
    /// The body is set to the `body` argument.
    /// The `max_response_bytes` is set to the `max_response_bytes` argument.
    /// The `max_response_bytes` argument is optional.
    /// The Default `Content-Type` header is set to `application/json`.
    /// The `Content-Type` header can be overwritten by using the `content_type` method.
    pub fn post(self, body: &str, max_response_bytes: Option<u64>) -> Self {
        self.method(HttpMethod::POST)
            .add_headers(vec![(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])
            .max_response_bytes(max_response_bytes)
            .body(body)
    }

    /// Updates the HTTP method.
    pub fn method(mut self, http_method: HttpMethod) -> Self {
        self.0.method = http_method;
        self
    }

    /// Updates the body.
    pub fn body(mut self, body: &str) -> Self {
        self.0.body = Some(body.as_bytes().to_vec());
        self
    }

    /// Adds HTTP headers for the request
    pub fn add_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.0
            .headers
            .extend(headers.iter().map(|(name, value)| HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            }));
        self
    }

    /// Sets the Content-Type header for the request
    pub fn content_type(mut self, content_type: &str) -> Self {
        // Remove any existing Content-Type headers
        self.0
            .headers
            .retain(|header| header.name != "Content-Type");

        // Add the new Content-Type header
        self.0.headers.push(HttpHeader {
            name: "Content-Type".to_string(),
            value: content_type.to_string(),
        });

        self
    }

    /// add query params to the URL
    pub fn add_query_params(mut self, params: Vec<(String, String)>) -> Self {
        let query_string = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        if self.0.url.contains('?') {
            self.0.url.push_str("&");
        } else {
            self.0.url.push_str("?");
        }
        self.0.url.push_str(&query_string);
        self
    }

    /// Updates the transform context of the request.
    pub fn transform_context(mut self, method: &str, context: Option<Vec<u8>>) -> Self {
        let context = TransformContext::from_name(method.to_string(), context.unwrap_or_default());

        self.0.transform = Some(context);
        self
    }

    /// Updates the max_response_bytes of the request.
    pub fn max_response_bytes(mut self, max_response_bytes: Option<u64>) -> Self {
        self.0.max_response_bytes = max_response_bytes;
        self
    }

    /// Calculate the cycle cost for this HTTP request
    pub fn calculate_cycle_cost(&self) -> u128 {
        HttpsOutcallCost::total(&self.0)
    }

    /// Wraps around `http_request` to issue a request to the `http_request` endpoint.
    pub async fn send(self) -> Result<HttpResponse, String> {
        let cycle_cost = self.calculate_cycle_cost();

        // You can log or use the cycle_cost here for further actions
        http_request(self.0, cycle_cost)
            .await
            .map(|(response,)| response)
            .map_err(|(_rejection_code, message)| message)
    }

    /// Wraps around `http_request_with_closure` to issue a request to the `http_request` endpoint with a transform closure.
    pub async fn send_with_closure(
        self,
        transform_func: impl FnOnce(HttpResponse) -> HttpResponse + 'static,
    ) -> Result<HttpResponse, String> {
        let cycle_cost = self.calculate_cycle_cost();
        // You can log or use the cycle_cost here for further actions

        http_request_with_closure(self.0, cycle_cost, transform_func)
            .await
            .map(|(response,)| response)
            .map_err(|(_rejection_code, message)| message)
    }
}
