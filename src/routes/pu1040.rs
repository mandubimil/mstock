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
    let (_, q_para, e_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_setting" =>
        {
            let query ="
            select   서버,
                     파일,
                     구분,
                     설명,
                     실행,
                     비고
            from     python_설정
            order by 파일,구분, 서버
            ";

            let qs ="
            select json_object(
                   '서버_' value 서버,
                   '파일_' value 파일,
                   '구분_' value 구분,
                   '설명_' value 설명,
                   '실행_' value 실행,
                   '비고_' value 비고
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }   
        "set_setting" =>
        {
            let text = "
            select count(*) cnt
            from python_설정
            where 서버 = :서버
              and 파일 = :파일
              and 구분 = :구분
            ";
            let select_one = db::select_one(db.clone(), text, q_para.clone());

            if select_one == "0"
            {
                let query = "
                insert into python_설정
                (서버, 파일, 구분, 설명, 실행, 비고)
                values
                (:서버, :파일, :구분, :설명, :실행, :비고)
                ";
    
                db::exec(db, &query, e_para)
            }
            else 
            {
                let query = "
                update python_설정
                set 설명 = :설명
                   ,실행 = :실행
                   ,비고 = :비고
                where 서버 = :서버
                and 파일 = :파일
                and 구분 = :구분
                  ";

                db::exec(db, &query, e_para)
            }                
        }   
        "del_setting" =>
        {
            let query = "
            delete python_설정
            where 서버 = :서버
            and 파일 = :파일
            and 구분 = :구분
            ";

            db::exec(db, &query, q_para)
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