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

    // let table_server_type: String = session.get::<String>("table_server_type")
    //     .unwrap_or(Some("err".to_string()))
    //     .unwrap_or("err".to_string());

    // println!("___ {}", table_server_type);

    let id = &job_id.0[..];
    let (_, mut q_para, e_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {        
        "get_jong" =>
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
            select  단축코드,
                    종목명,
                    etf구분,
                    기준가,
                    시가총액, 
                    상장주식,
                    외국인보유,
                    per,
                    pbr,
                    자본금
            from   종목마스터
            where    구분 = decode(:시장, 'ZZ', 구분, :시장)
              and    etf구분 = decode(:etf, 'ZZ', etf구분, :etf)
              {} {}
            order by 단축코드     
            ", sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   'etf' value etf구분,
                   '기준가' value 기준가,
                   '시가총액' value 시가총액,
                   '상장주식' value 상장주식,
                   '외국인보유_' value 외국인보유,
                   'per_' value per,
                   'pbr_' value pbr,
                   '자본금' value 자본금
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_uj" =>
        {
            let query = "
            select   코드 업종코드, get_업종명(코드) 업종명,
                     내용 단축코드, get_종목명(내용) 종목명
            from     테마업종
            where    구분 = '업종내역'
            order by 업종명
            ";

            let qs ="
            select json_object(
                   '업종코드_' value 업종코드,
                   '업종명_' value 업종명,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }        
        "get_tm" =>
        {
            let query = "
            select   코드 테마코드, get_테마명(코드) 테마명,
                     내용 단축코드, get_종목명(내용) 종목명
            from     테마업종
            where    구분 = '테마내역'
            order by 테마명
            ";

            let qs ="
            select json_object(
                   '테마코드_' value 테마코드,
                   '테마명_' value 테마명,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }       
        "get_uj_tm" =>
        {
            let query = "
            select     업종.코드 업종코드,
                        get_업종명(업종.코드) 업종명,
                        테마.코드 테마코드,
                        get_테마명(테마.코드) 테마명,
                        테마.내용 단축코드,
                        get_종목명(테마.내용) 종목명
            from    (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '테마내역'
                        ) 테마,
                        (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '업종내역'
                        ) 업종
            where  테마.내용 = 업종.내용
            order by 업종명, 테마명, 종목명
            ";

            let qs ="
            select json_object(
                   '업종코드_' value 업종코드,
                   '업종명_' value 업종명,
                   '테마코드_' value 테마코드,
                   '테마명_' value 테마명,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_tm_uj" =>
        {
            let query = "
            select      테마.코드 테마코드,
                        get_테마명(테마.코드) 테마명,
                        업종.코드 업종코드,
                        get_업종명(업종.코드) 업종명,
                        테마.내용 단축코드,
                        get_종목명(테마.내용) 종목명
            from    (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '테마내역'
                        ) 테마,
                        (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '업종내역'
                        ) 업종
            where  테마.내용 = 업종.내용
            order by 테마명, 업종명, 종목명
            ";

            let qs ="
            select json_object(
                   '테마코드_' value 테마코드,
                   '테마명_' value 테마명,
                   '업종코드_' value 업종코드,
                   '업종명_' value 업종명,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }            
        _ =>
        {
            db::get_no_job_id()
        }
    };

    //println!("{:#?}", result_json);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result_json.dump()))
}