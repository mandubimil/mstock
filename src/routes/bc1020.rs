use actix_web::{web, Error, HttpResponse};
use bytes::{Bytes};
use r2d2_oracle::OracleConnectionManager;
use r2d2::Pool;
use actix_session::Session;

use super::super::comm::*;

pub async fn post_job(
    body: Bytes,
    job_id: web::Path<(String,)>,
    db: web::Data<Pool<OracleConnectionManager>>,
    session: Session,
) -> Result<HttpResponse, Error> {

    if util::check_session(&session) { return Ok(HttpResponse::Ok().body("who?")) };

    let id = &job_id.0[..];
    let (_, mut q_para, e_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_si" => 
        {
            let query = "
            SELECT  JSON_OBJECT('코드' VALUE 코드, '코드명' VALUE 코드명)
            from    (
                    select   '1' 코드, '코스피' 코드명 from dual union all
                    select   '2' 코드, '코스닥' 코드명 from dual
            )
            ";

            db::select_json(db, &query, q_para)
        }
        "get_etf" => 
        {
            let query = "
            SELECT  JSON_OBJECT('코드' VALUE 코드, '코드명' VALUE 코드명)
            from    (
                    select   '0' 코드, '일반' 코드명 from dual union all
                    select   '1' 코드, 'etf' 코드명 from dual union all
                    select   '2' 코드, 'etn' 코드명 from dual
            )
            ";

            db::select_json(db, &query, q_para)
        }
        "get_uj" => 
        {
            let query = "
            SELECT  JSON_OBJECT('코드' VALUE 코드, '코드명' VALUE 내용)
            from    (
                    select 코드, 내용 from 테마업종 where 구분 = '업종코드'
            )
            ";

            db::select_json(db, &query, q_para)
        }
        "get_tm" => 
        {
            let query = "
            SELECT  JSON_OBJECT('코드' VALUE 코드, '코드명' VALUE 내용)
            from    (
                    select 코드, 내용 from 테마업종 where 구분 = '테마코드'
            )
            ";

            db::select_json(db, &query, q_para)
        }
        "get_jong_list" => 
        {
            let mut sql_jo_1 = String::new();
            if e_para["테마"] != "ZZ"
            {
                sql_jo_1 = " and 단축코드 in (select 내용 from 테마업종 where 구분='테마내역' and 코드 = :테마)".to_string();
                q_para.insert("테마".to_string(), e_para["테마"].clone());
            }
 
            let mut sql_jo_2 = String::new();
            if e_para["업종"] != "ZZ"
            {
                sql_jo_2 = " and 단축코드 in (select 내용 from 테마업종 where 구분='업종내역' and 코드 = :업종)".to_string();
                q_para.insert("업종".to_string(), e_para["업종"].clone());
            }

            let query = format!("
            SELECT  JSON_OBJECT('코드' VALUE 단축코드, '코드명' VALUE 종목명)
            from    (
                    select   단축코드, get_종목명(단축코드) 종목명
                    FROM     종목마스터
                    where    구분 = decode(:시장, 'ZZ', 구분, :시장)
                      and    etf구분 = decode(:etf구분, 'ZZ', etf구분, :etf구분)
                    {} {}
                    order by 단축코드
            )
            ", sql_jo_1, sql_jo_2);            

            db::select_json(db, &query, q_para)
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