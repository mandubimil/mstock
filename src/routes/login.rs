use actix_web::{web, Error, HttpResponse};
use bytes::{Bytes};
use r2d2_oracle::OracleConnectionManager;
use r2d2::Pool;
use json::JsonValue;
use std::collections::HashMap;
use actix_session::Session;

use super::super::comm::*;

pub async fn post_job(
    body: Bytes,
    job_id: web::Path<(String,)>,
    db: web::Data<Pool<OracleConnectionManager>>,
    session: Session,
) -> Result<HttpResponse, Error> {

    let id = &job_id.0[..];
    let (j_para, para, _) = util::body_to_hash_jo(body);
    let mut result_json : JsonValue = JsonValue::new_object();

    match id
    {
        "gogo" => 
        {
            let query = "
            SELECT   JSON_OBJECT('결과' VALUE DECODE(COUNT(아이디), 1, 'PASS', 'NO')) JSON
            FROM     사용자
            WHERE    아이디 = :아이디
              AND    비밀번호 = :비밀번호
            ";

            result_json = db::select_json(db, query, para);
            let check_str: HashMap<String, String> = serde_json::from_str(&result_json["data"][0].dump()).unwrap();

            if check_str["결과"] == "PASS".to_string()
            {
                session.set("mstock_login", "ok_good_pass").unwrap();    
                session.set("server_type", &j_para["서버타입"]).unwrap();    

                if j_para["서버타입"] == "hts"
                {
                    session.set("table_server_type", "주식").unwrap();
                }
                else 
                {
                    session.set("table_server_type", "모의").unwrap();
                }                
            }
            else
            {
                session.set("mstock_login", "no").unwrap();    
            }            
        }
        _ =>
        {
            session.set("mstock_login", "no").unwrap();    
        }
    }     

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result_json.dump()))
}

