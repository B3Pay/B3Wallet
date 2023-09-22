use b3_utils::{
    http::HttpRequest,
    log_cycle,
    logs::{export_log, LogEntry},
};
use ic_cdk::{
    api::management_canister::http_request::{HttpHeader, HttpResponse, TransformArgs},
    query, update,
};

#[update]
async fn http_get(url: String, max_response_bytes: Option<u64>) -> String {
    log_cycle!("Calling http_get");

    let request = HttpRequest::new(url).get(max_response_bytes);

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request
        .transform_context("get_transform", None)
        .send()
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_get_with_closure(url: String, max_response_bytes: u64) -> String {
    log_cycle!("Calling http_get_with_closure");

    let request = HttpRequest::new(url).get(Some(max_response_bytes));

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request
        .send_with_closure(|response| HttpResponse {
            status: response.status,
            body: response.body,
            headers: headers(),
            ..Default::default()
        })
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_get_with_features(
    url: String,
    max_response_bytes: Option<u64>,
    custom_headers: Option<Vec<(String, String)>>,
    query_params: Option<Vec<(String, String)>>,
) -> String {
    log_cycle!("Calling http_get_with_features");

    // Initialize the HttpRequest
    let mut request = HttpRequest::new(url.clone());

    // Add query parameters if provided
    if let Some(params) = query_params {
        request = request.add_query_params(params);
    }

    // Create a GET request
    request = request.get(max_response_bytes);

    // Add custom headers if provided
    if let Some(headers) = custom_headers {
        request = request.add_headers(headers);
    }

    // Calculate cycle cost
    let cycle_cost = request.calculate_cycle_cost();
    log_cycle!("Calculated cycle cost: {}", cycle_cost);

    // Add a transform context and send the request
    let response_result = request
        .transform_context("get_transform", None)
        .send()
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("Response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_head(url: String, max_response_bytes: Option<u64>) -> String {
    log_cycle!("Calling http_head");

    let request = HttpRequest::new(url).head(max_response_bytes);

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request
        .transform_context("head_transform", None)
        .send()
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());

            format!("{:?}", response)
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_post(url: String, json_string: String, max_response_bytes: Option<u64>) -> String {
    log_cycle!("Calling http_post");

    let request = HttpRequest::new(url).post(&json_string, max_response_bytes);

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request
        .transform_context("post_transform", None)
        .send()
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_post_with_features(
    url: String,
    json_string: String,
    max_response_bytes: Option<u64>,
    custom_content_type: Option<String>,
    custom_headers: Option<Vec<(String, String)>>,
    query_params: Option<Vec<(String, String)>>,
) -> String {
    log_cycle!("Calling http_post_with_features");

    // Initialize the HttpRequest
    let mut request = HttpRequest::new(url.clone());

    // Add query parameters if provided
    if let Some(params) = query_params {
        request = request.add_query_params(params);
    }

    // Create a POST request
    request = request.post(&json_string, max_response_bytes);

    // Overwrite Content-Type if provided
    if let Some(content_type) = custom_content_type {
        request = request.content_type(&content_type);
    }

    // Add custom headers if provided
    if let Some(headers) = custom_headers {
        request = request.add_headers(headers);
    }

    // Calculate cycle cost
    let cycle_cost = request.calculate_cycle_cost();
    log_cycle!("Calculated cycle cost: {}", cycle_cost);

    // Add a transform context and send the request
    let response_result = request
        .transform_context("post_transform", None)
        .send()
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("Response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[update]
async fn http_post_with_closure(
    url: String,
    json_string: String,
    max_response_bytes: u64,
) -> String {
    log_cycle!("Calling http_post_with_closure");

    let request = HttpRequest::new(url).post(&json_string, Some(max_response_bytes));

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request
        .send_with_closure(|response| HttpResponse {
            status: response.status,
            body: response.body,
            headers: headers(),
            ..Default::default()
        })
        .await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

#[query]
fn get_transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status,
        body: raw.response.body,
        headers: headers(),
        ..Default::default()
    }
}

#[query]
fn head_transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse { ..raw.response }
}

#[query]
fn post_transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status,
        body: raw.response.body,
        headers: headers(),
        ..Default::default()
    }
}

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

fn headers() -> Vec<HttpHeader> {
    vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ]
}

ic_cdk::export_candid!();
