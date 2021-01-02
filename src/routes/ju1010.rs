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

    // println!("___ {}", table_server_type);

    let id = &job_id.0[..];
    let (_,para, _) = util::body_to_hash_jo(body);

    let result_json = match id
    {        
        "get_bo_for_modify" =>
        {
            let query =format!("
            select  매매순번,
                    단축코드,
                    get_종목명(단축코드) 종목명,
                    매수일자, 
                    매수수량, 실매수수량,
                    매수단가, 실매수단가,
                    매도일자,
                    매도수량, 실매도수량,
                    매도단가, 실매도단가,
                    매수조건명,
                    매수조건내역,
                    매도조건명,
                    매도조건내역
            from   {}_매매
            where  실매수수량 - 실매도수량 <> 0
              and  단축코드 = :단축코드
            order by 매수일자 desc
     
            ", table_server_type);

            let qs ="
            select json_object(
                   '매매순번' value 매매순번,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '매수일자' value 매수일자,
                   '매수수량' value 매수수량,
                   '실매수수량' value 실매수수량,
                   '매수단가' value 매수단가,
                   '실매수단가' value 실매수단가,
                   '매도일자' value 매도일자,
                   '매도수량' value 매도수량,
                   '실매도수량' value 실매도수량,
                   '매도단가' value 매도단가,
                   '실매도단가' value 실매도단가,
                   '매수조건명' value 매수조건명,
                   '매수조건내역' value 매수조건내역,
                   '매도조건명' value 매도조건명,
                   '매도조건내역' value 매도조건내역
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }
        "get_list_bo" =>
        {
            let query =format!("
            select 단축코드,
                    get_종목명(단축코드) 종목명,
                    sum(실매수수량 - 실매도수량) 수량
            from   {}_매매
            where  실매수수량 - 실매도수량 <> 0
            group by 단축코드, get_종목명(단축코드)
            order by 단축코드
     
            ", table_server_type);

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '수량' value 수량,
                   '주식서버' value 0,
                   '차이' value ''
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }
        "get_list_meme" =>
        {
            let query =format!("
            select '매수' 구분,
                    단축코드,
                    get_종목명(단축코드) 종목명,
                    매수일자 일자,
                    실매수수량 수량,
                    실매수단가 단가,
                    '' 수익
            from {}_매매
            where 매수일자 between :시작일자 and :종료일자
            
            union all
            
            select '매도' 구분,
                    단축코드,
                    get_종목명(단축코드) 종목명,
                    매도일자,
                    실매도수량,
                    실매도단가,
                    to_char(round((실매도단가/실매수단가)*100)) 수익
            from {}_매매
            where 매도일자 between :시작일자 and :종료일자
              and 실매도수량 <> 0
            
            order by 일자 desc, 단축코드
            ", table_server_type, table_server_type);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '구분' value 구분,
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '수량' value 수량,
                   '수익' value 수익,
                   '단가' value 단가
                   )
            from   (";
            let qe ="
                )";

            //println!("{:#?}", para);

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }
        "del_meme" =>
        {
            let query =format!("
            delete from {}_매매
            where 매매순번 = :매매순번
            ", table_server_type);

            db::exec(db, &query, para)
        }
        "update_meme" =>
        {
            let query =format!("
            update {}_매매
            set 매수수량 = :매수수량
               ,실매수수량 = :실매수수량
               ,매도수량 = :매도수량
               ,실매도수량 = :실매도수량
               ,매수단가 = :매수단가
               ,실매수단가 = :실매수단가
               ,매도단가 = :매도단가
               ,실매도단가 = :실매도단가
               ,매도조건명 = :매도조건명
               ,매도조건내역 = :매도조건내역
            where 매매순번 = :매매순번
            ", table_server_type);

            db::exec(db, &query, para)
        }
        "new_mesu" =>
        {
            let query =format!("
            insert into {}_매매
                    (매매순번, 단축코드, 
                    매수일자, 매수수량, 매수단가, 실매수수량, 실매수단가, 매수조건명, 매수조건내역,
                    매도일자, 매도수량, 매도단가, 실매도수량, 실매도단가, 매도조건명, 매도조건내역,
                    현재주가)
            values (
                (select sum(순번) from (select max(매매순번) 순번 from {}_매매 union all select 1 from dual)),
                :단축코드,

                to_char(sysdate, 'YYYYMMDD'),
                :매수수량,
                :매수단가,
                :매수수량,
                :매수단가,
                '조정입력',
                '{}',

                '',
                0,
                0,
                0,
                0,
                '가격비교',
                '{}',

                0
            )
            ", table_server_type, table_server_type, "{}", "{\"상승판매\":\"120\", \"하락판매\":\"90\", \"보유일\":\"30\"}");

            db::exec(db, &query, para)
        }
        "new_meme" =>
        {
            let query =format!("
            insert into {}_매매
                    (매매순번, 단축코드, 
                    매수일자, 매수수량, 매수단가, 실매수수량, 실매수단가, 매수조건명, 매수조건내역,
                    매도일자, 매도수량, 매도단가, 실매도수량, 실매도단가, 매도조건명, 매도조건내역,
                    현재주가)
            values (
                (select sum(순번) from (select max(매매순번) 순번 from {}_매매 union all select 1 from dual)),
                (select 단축코드 from {}_매매 where 매매순번 = :매매순번),
                
                (select 매수일자 from {}_매매 where 매매순번 = :매매순번),
                (select 매수수량 from {}_매매 where 매매순번 = :매매순번),
                (select 매수단가 from {}_매매 where 매매순번 = :매매순번),
                (select 실매수수량 from {}_매매 where 매매순번 = :매매순번),
                (select 실매수단가 from {}_매매 where 매매순번 = :매매순번),
                (select 매수조건명 from {}_매매 where 매매순번 = :매매순번),
                (select 매수조건내역 from {}_매매 where 매매순번 = :매매순번),

                (select 매도일자 from {}_매매 where 매매순번 = :매매순번),
                (select 매도수량 from {}_매매 where 매매순번 = :매매순번),
                (select 매도단가 from {}_매매 where 매매순번 = :매매순번),
                (select 실매도수량 from {}_매매 where 매매순번 = :매매순번),
                (select 실매도단가 from {}_매매 where 매매순번 = :매매순번),
                (select 매도조건명 from {}_매매 where 매매순번 = :매매순번),
                (select 매도조건내역 from {}_매매 where 매매순번 = :매매순번),

                0
            )
            ", table_server_type, table_server_type, table_server_type
            , table_server_type, table_server_type, table_server_type, table_server_type, table_server_type, table_server_type, table_server_type
            , table_server_type, table_server_type, table_server_type, table_server_type, table_server_type, table_server_type, table_server_type);

            db::exec(db, &query, para)
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