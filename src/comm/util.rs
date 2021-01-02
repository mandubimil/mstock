use actix_session::Session;
use bytes::Bytes;
use json::JsonValue;
use std::collections::HashMap;

pub fn body_to_hash_jo(
    body: Bytes,
) -> (
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, String>,
) {
    let body_str = std::str::from_utf8(&body).unwrap();

    let result = json::parse(body_str);
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string()},
    };

    let mut q_para: HashMap<String, String> = HashMap::new();
    if !injson["q_para"].is_empty() {
        q_para = serde_json::from_str(&injson["q_para"].dump()).unwrap();
    }

    let mut j_para: HashMap<String, String> = HashMap::new();
    if !injson["j_para"].is_empty() {
        j_para = serde_json::from_str(&injson["j_para"].dump()).unwrap();
    }

    let mut e_para: HashMap<String, String> = HashMap::new();
    if !injson["e_para"].is_empty() {
        e_para = serde_json::from_str(&injson["e_para"].dump()).unwrap();
    }

    (j_para, q_para, e_para)
}

pub fn check_session(session: &Session) -> bool {
    if let Some(mstock_login) = session.get::<String>("mstock_login").unwrap() {
        if mstock_login == "ok_good_pass".to_string() {
            return false;
        } else {
            println!("2 {}", mstock_login);
            return true;
        }
    }

    return true;
}
