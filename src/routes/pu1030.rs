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
    let table_server_type: String = session.get::<String>("table_server_type")
        .unwrap_or(Some("err".to_string()))
        .unwrap_or("err".to_string());


    let id = &job_id.0[..];
    let (_, q_para, _) = util::body_to_hash_jo(body);


    let result_json = match id
    {        
        "insert_group" =>
        {
            let text = "
            select count(*) cnt
            from 종목관리
            where 그룹명 = :그룹명
              and 단축코드 = :단축코드
            ";
            let select_one = db::select_one(db.clone(), text, q_para.clone());

            if select_one == "0"
            {
                let query = "
                insert into 종목관리
                (그룹명, 단축코드, 등록주가, 등록일자)
                values
                (:그룹명, :단축코드
                  , (
                        select   수정종가
                        from     일봉
                        where    단축코드 = :단축코드
                          and    일자 = (select max(일자) from 일봉 where 단축코드 = :단축코드)
                    )                
                , to_char(sysdate, 'YYYYMMDD'))
                ";
    
                db::exec(db, &query, q_para)        
            }
            else 
            {
                println!("동일 그룹에 동일한 단축코드가 있습니다.");
                db::get_no_job_id()
            }
        }
        "del_group" =>
        {
            let query = "
            delete from 종목관리
            where 그룹명 = :그룹명
            ";

            db::exec(db, &query, q_para)
        }
        "del_group_jong" =>
        {
            let query = "
            delete from 종목관리
            where 그룹명 = :그룹명
              and 단축코드 = :단축코드
            ";

            db::exec(db, &query, q_para)
        }
        "get_group" =>
        {
            let query = "
            select 그룹명, max(등록일자) 등록일자 from 종목관리 group by 그룹명 order by 그룹명
            ";

            let qs ="
            select json_object(
                   '그룹명' value 그룹명,
                   '등록일자_' value 등록일자
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }  
        "get_ju" =>
        {
            let query =format!("
            select 예약순번, 단축코드, get_종목명(단축코드) 종목명, 수량, 
                   decode(매매구분, '1', '매도', '2', '매도') 매매구분, 
                   decode(금액구분, '00', '지정가', '03', '시장가') 금액구분,
                   주문번호
            from   {}_예약
            where  주문번호 = '0'
              or   예약일자 = to_char(sysdate, 'YYYYMMDD')
            order by 예약일자 desc, 예약시간 desc     
            ", table_server_type);            

            let qs ="
            select json_object(
                   '예약순번' value 예약순번,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '수량' value 수량,
                   '매매구분' value 매매구분,
                   '금액구분' value 금액구분,
                   '주문번호_' value 주문번호
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }            
        "get_group_jong" =>
        {
            let query = "
            select 단축코드,
                   get_종목명(단축코드) 종목명,
                   등록주가,
                   등록일자,
                   비고1,
                   비고2,
                   비고3
            from   종목관리
            where  그룹명 = :그룹명
            ";

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '등록주가'  value 등록주가,                   
                   '등록일자' value 등록일자,
                   '비고1' value 비고1,
                   '비고2' value 비고2,
                   '비고3' value 비고3
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

    // println!("{:#?}", result_json);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result_json.dump()))
        
}