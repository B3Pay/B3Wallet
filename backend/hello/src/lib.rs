#[ic_cdk_macros::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    candid::export_service!();

    let candid = format!("{}", __export_service());

    let mut file = std::fs::File::create("./hello.did").unwrap();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
