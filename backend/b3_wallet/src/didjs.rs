use b3_helper_lib::types::{B3Path, HeaderField, HttpRequest, HttpResponse};
use b3_wallet_lib::types::PathTrait;
use ic_cdk::{
    export::candid::{candid_method, check_prog, types::subtype, types::Type, IDLProg, TypeEnv},
    query,
};

#[query]
#[candid_method(query)]
fn did_to_js(prog: String) -> Option<String> {
    let ast = prog.parse::<IDLProg>().ok()?;
    let mut env = TypeEnv::new();
    let actor = check_prog(&mut env, &ast).ok()?;
    let res = ic_cdk::export::candid::bindings::javascript::compile(&env, &actor);
    Some(res)
}

#[query]
#[candid_method(query)]
fn binding(prog: String, lang: String) -> Option<String> {
    use ic_cdk::export::candid::bindings;
    let ast = prog.parse::<IDLProg>().ok()?;
    let mut env = TypeEnv::new();
    let actor = check_prog(&mut env, &ast).ok()?;
    let res = match lang.as_str() {
        "ts" => bindings::typescript::compile(&env, &actor),
        "mo" => bindings::motoko::compile(&env, &actor),
        "installed_did" => {
            let actor = if let Some(Type::Class(_, ty)) = actor {
                Some(*ty)
            } else {
                actor
            };
            bindings::candid::compile(&env, &actor)
        }
        _ => return None,
    };
    Some(res)
}

#[query]
#[candid_method(query)]
fn subtype(new: String, old: String) -> Result<(), String> {
    let new = new.parse::<IDLProg>().unwrap();
    let old = old.parse::<IDLProg>().unwrap();
    let mut new_env = TypeEnv::new();
    let mut old_env = TypeEnv::new();
    let new_actor = check_prog(&mut new_env, &new).unwrap().unwrap();
    let old_actor = check_prog(&mut old_env, &old).unwrap().unwrap();
    let mut gamma = std::collections::HashSet::new();
    let old_actor = new_env.merge_type(old_env, old_actor);
    subtype::subtype(&mut gamma, &new_env, &new_actor, &old_actor).or_else(|e| Err(e.to_string()))
}

#[query]
#[candid_method(query)]
fn http_request(request: HttpRequest) -> HttpResponse {
    //TODO add /canister_id/ as endpoint when ICQC is available.
    let path = B3Path::new(request.url.as_str());

    if let Some(bytes) = path.retrieve() {
        HttpResponse {
            status_code: 200,
            headers: vec![
                //HeaderField("Content-Encoding".to_string(), "gzip".to_string()),
                HeaderField("Content-Length".to_string(), format!("{}", bytes.len())),
                HeaderField("Cache-Control".to_string(), format!("max-age={}", 600)),
            ],
            body: bytes.to_vec(),
        }
    } else {
        HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: path.to_vec(),
        }
    }
}
