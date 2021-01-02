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
    let (j_para, para, empty_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_query" =>
        {
            let query = "
            select 분류1 from 쿼리 order by 분류1
            ";

            let qs ="
            select json_object(
                   '분류1' value 분류1
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, empty_para)
        }  
        "get_query_content" =>
        {
            let query = "
            select 내용1 from 쿼리 where 분류1 = :분류1
            ";

            let qs ="
            select json_object(
                   '내용1' value 내용1
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }  
        "save_query" =>
        {
            let text = "
            select count(*) cnt
            from 쿼리
            where 분류1 = :분류1
            ";
            let select_one = db::select_one(db.clone(), text, j_para);

            if select_one == "0"
            {
                let query = "
                insert into 쿼리
                (분류1, 내용1)
                values
                (:분류1, :내용1)
                ";
    
                db::exec(db, &query, para)        
            }
            else 
            {
                let query = "
                update 쿼리
                set 내용1 = :내용1
                where 분류1 = :분류1
                ";
    
                db::exec(db, &query, para)        
            }
        }
        "del_query" =>
        {
            let query = "
            delete 쿼리
            where 분류1 = :분류1
            ";

            db::exec(db, &query, para)        
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