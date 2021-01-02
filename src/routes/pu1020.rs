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
        "update_group_jong" =>
        {
            let query = "
            update 종목관리
            set  비고1 = :비고1
                ,비고2 = :비고2
                ,비고3 = :비고3
                ,그룹명 = :수정_그룹명
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
        "get_group_jong" =>
        {
            let query = "
            select 종목관리.단축코드,
                   get_종목명(종목관리.단축코드) 종목명,
                   종목관리.등록주가,
                   마지막일봉.수정종가,
                   round((마지막일봉.수정종가/종목관리.등록주가)*100,2) 등락,
                   종목관리.등록일자,
                   종목관리.비고1,
                   종목관리.비고2,
                   종목관리.비고3
            from   종목관리,
                   마지막일봉
            where  그룹명 = :그룹명
              and  종목관리.단축코드 = 마지막일봉.단축코드
            ";

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '등록주가'  value 등록주가,                   
                   '최종가'  value 수정종가,                   
                   '등락' value 등락,
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
        "get_list_bo" =>
        {
            let query =format!("
            select 매매순번,
                   단축코드,
                    get_종목명(단축코드) 종목명,
                    실매수수량 - 실매도수량 수량
            from   {}_매매
            where  실매수수량 - 실매도수량 <> 0
            order by 단축코드
     
            ", table_server_type);

            let qs ="
            select json_object(
                   '매매순번' value 매매순번,
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
            db::select_json(db, &query_tot, q_para)
        }           
        "get_reservation" =>
        {
            let query =format!("
            select *
            from (
                     select '매도' 구분,
                            예약.예약순번,
                            예약.단축코드,
                            get_종목명(예약.단축코드) 종목명,
                            예약.예약일자,
                            예약.예약시간,
                            예약.주문번호,
                            예약.수량 예약수량,
                            예약.단가 예약단가,
                            매매.매도일자 매매일자,
                            매매.실매도수량 실매매수량,
                            매매.실매도단가 실매매단가,
                            decode(예약.금액구분, '00', '지정가', '03', '시장가') 금액구분
                     from {}_예약 예약,
                          {}_매매 매매
                     where 예약.주문번호 = 매매.매도주문번호(+)
                       and 예약.매매구분 = '1'
                       and 예약.예약일자 between :시작일자 and :종료일자
                     union all
                     select '매도' 구분,
                            예약.예약순번,
                            예약.단축코드,
                            get_종목명(예약.단축코드) 종목명,
                            예약.예약일자,
                            예약.예약시간,
                            예약.주문번호,
                            예약.수량 예약수량,
                            예약.단가 예약단가,
                            매매.매수일자 매매일자,
                            매매.실매수수량 실매매수량,
                            매매.실매수단가 실매매단가,
                            decode(예약.금액구분, '00', '지정가', '03', '시장가') 금액구분
                     from {}_예약 예약,
                          {}_매매 매매
                     where 예약.주문번호 = 매매.매수주문번호(+)
                       and 예약.매매구분 = '2'
                       and 예약.예약일자 between :시작일자 and :종료일자            
                 )
            order by 예약일자 desc, 예약시간 desc
            ", table_server_type, table_server_type, table_server_type, table_server_type);

            let qs ="
            select json_object(
                '구분' value 구분,
                '예약순번' value 예약순번,
                '단축코드' value 단축코드,
                '종목명' value 종목명,
                '예약일자_' value 예약일자,
                '예약시간_' value 예약시간,
                '주문번호' value 주문번호,
                '예약수량' value 예약수량,
                '예약단가' value 예약단가,
                '매매일자' value 매매일자,
                '실매매수량' value 실매매수량,
                '실매매단가' value 실매매단가,
                '금액구분' value 금액구분
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