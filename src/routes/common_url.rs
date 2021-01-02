use actix_web::{web, Error, HttpResponse};
use bytes::{Bytes};
use r2d2_oracle::OracleConnectionManager;
use r2d2::Pool;
use actix_session::Session;

use reqwest;

use super::super::comm::*;

pub async fn post_job(
    body: Bytes,
    job_id: web::Path<(String,)>,
    db: web::Data<Pool<OracleConnectionManager>>,
    session: Session,
) -> Result<HttpResponse, Error> {

    if util::check_session(&session) { return Ok(HttpResponse::Ok().body("who?")) };

    let id = &job_id.0[..];
    let (_,para,_) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "combobox_uj" =>
        {
            let query ="
            select   json_object
                     (
                     'id' value id,
                     'value' value value
                     )
            from     (
                     select 'ZZ' id, '전체' value from dual union all select 코드, 내용 from 테마업종 where 구분 = '테마코드' order by value
                     )
            ";

            db::select_json(db, query, para)
        }
        "get_jong_list" => 
        {
            let query = "
            SELECT   JSON_OBJECT('단축코드' VALUE 단축코드, '종목명' VALUE 종목명) JSON
            FROM     종목마스터
            ";

            db::select_json(db, query, para)
        }
        "get_jong_name" => 
        {
            let query = "
            SELECT   JSON_OBJECT('단축코드' VALUE 단축코드, '종목명' VALUE 종목명) JSON
            FROM     종목마스터
            WHERE    단축코드 = :단축코드
            ";

            db::select_json(db, query, para)
        }
        _ =>
        {
            db::get_no_job_id()
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result_json.dump()))
}



pub async fn get_job(
    job_id: web::Path<(String,)>,
    body: Bytes,
    db: web::Data<Pool<OracleConnectionManager>>,
    session: Session,
) -> Result<HttpResponse, Error> {

    if util::check_session(&session) { return Ok(HttpResponse::Ok().body("who?")) };

    let id = &job_id.0[..];
    let (_, q_para, _) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "combobox_uj" =>
        {
            let query ="
            select   json_object
                     (
                     'id' value id,
                     'value' value value
                     ) json
            from     (
                     select 'ZZ' id, '전체' value from dual union all select * from (select 코드, 내용 from 테마업종 where 구분 = '업종코드' order by 내용)
                     )
            ";

            db::select_json(db, query, q_para)
        }
        "combobox_tm" =>
        {
            let query ="
            select   json_object
                     (
                     'id' value id,
                     'value' value value
                     ) json
            from     (
                     select 'ZZ' id, '전체' value from dual union all select * from (select 코드, 내용 from 테마업종 where 구분 = '테마코드' order by 내용)
                     )
            ";

            db::select_json(db, query, q_para)
        }
        _ =>
        {
            db::get_no_job_id()
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result_json.dump()))
}



pub async fn python_exe(
    body: Bytes,
    session: Session,
) -> Result<HttpResponse, Error> {

    if util::check_session(&session) { return Ok(HttpResponse::Ok().body("who?")) };
    let server_type: String = session.get::<String>("server_type")
        .unwrap_or(Some("err".to_string()))
        .unwrap_or("err".to_string());

    let para = std::str::from_utf8(&body).unwrap();

    let paras = [("server_type", &server_type[..]), ("mk", "qhrhtlvek11!"), ("para", para)];

    let client = reqwest::Client::new();
    let response = client.post("http://192.168.0.112:1204/python_exe")
        .form(&paras)
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}
