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
    let (_, para, e_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_chart" =>
        {
            let query =format!("
            select  일자,
                    주가,
                    decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위
            from    (
                    select  일자,
                            round(avg((수정시가 / 중간주가 ) * 1000)) 주가
                    from    일봉,
                            (
                            select   단축코드, round((min(수정시가) + max(수정시가)) / 2) 중간주가
                            from     일봉
                            where    단축코드 in ({})
                            and 일자 between :시작일자 and :종료일자
                            group by 단축코드
                            ) 기준
                    where   일봉.단축코드 in ({})
                      and   일자 between :시작일자 and :종료일자
                      and   일봉.단축코드 = 기준.단축코드
                    group by 일자
                    )
            order by 일자
            ", e_para["단축코드"], e_para["단축코드"]);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '주가' value 주가,
                   '월단위' value 월단위
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }        
        "get_chart_basic" =>
        {
            let query =format!("
            select  min(주가) 최저주가, max(주가) 최고주가
            from    (
                    select  일자,
                            round(avg((수정시가 / 중간주가 ) * 1000)) 주가
                    from    일봉,
                            (
                            select   단축코드, round((min(수정시가) + max(수정시가)) / 2) 중간주가
                            from     일봉
                            where    단축코드 in ({})
                            and 일자 between :시작일자 and :종료일자
                            group by 단축코드
                            ) 기준
                    where   일봉.단축코드 in ({})
                      and   일자 between :시작일자 and :종료일자
                      and   일봉.단축코드 = 기준.단축코드
                    group by 일자
                    )
            ", e_para["단축코드"], e_para["단축코드"]);

            let qs ="
            select json_object(
                   '최저주가' value 최저주가,
                   '최고주가' value 최고주가
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }       
        "get_ger_chart" =>
        {
            let query =format!("
            select   일자,
                     sum(누적거래량) 누적거래량,
                     sum(개인) 개인,
                     sum(기관) 기관,
                     sum(외국인) 외국인,
                     sum(외국계) 외국계,
                     sum(프로그램) 프로그램,
                     SUM(SUM(외국계)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국계_누적,
                     SUM(SUM(외국인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국인_누적,
                     SUM(SUM(기관)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 기관_누적,
                     SUM(SUM(개인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 개인_누적,
                     SUM(SUM(프로그램)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 프로그램_누적,
                     decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위
            from     거래
            where    단축코드 in ({})
              and 일자 between :시작일자 and :종료일자
            group by 일자
            order by 일자
            ", e_para["단축코드"]);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '누적거래량' value 누적거래량,
                   '개인' value 개인,
                   '기관' value 기관,
                   '외국인' value 외국인,
                   '외국계' value 외국계,
                   '프로그램' value 프로그램,
                   '개인누적' value 개인_누적,
                   '기관누적' value 기관_누적,
                   '외국계누적' value 외국계_누적,
                   '외국인누적' value 외국인_누적,
                   '프로그램누적' value 프로그램_누적,
                   '월단위' value 월단위
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }        
        "get_ger_chart_basic" =>
        {
            let query =format!("
            select   max(개인) 개인max,
                     min(개인) 개인min,
                     max(기관) 기관max,
                     min(기관) 기관min,
                     max(외국인) 외국인max,
                     min(외국인) 외국인min,
                     max(외국계) 외국계max,
                     min(외국계) 외국계min,
                     max(프로그램) 프로그램max,
                     min(프로그램) 프로그램min
            from     (select 일자, 
                            SUM(SUM(외국계)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국계,
                            SUM(SUM(외국인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국인,
                            SUM(SUM(기관)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 기관,
                            SUM(SUM(개인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 개인,
                            SUM(SUM(프로그램)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 프로그램
                     from     거래
                     where    단축코드 in ({})
                       and 일자 between :시작일자 and :종료일자
                     group by 일자                 
                      )
            ", e_para["단축코드"]);

            let qs ="
            select json_object(
                   '개인max' value 개인max,
                   '개인min' value 개인min,
                   '외국인max' value 외국인max,
                   '외국인min' value 외국인min,
                   '외국계max' value 외국계max,
                   '외국계min' value 외국계min,
                   '기관max' value 기관max,
                   '기관min' value 기관min,
                   '프로그램max' value 프로그램max,
                   '프로그램min' value 프로그램min
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
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



// select  일자,
//         주가,
//         decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위
// from    (
//         select  일자,
//                 round(avg((수정시가 / 중간주가 ) * 1000)) 주가
//         from    일봉,
//                 (
//                 select   단축코드, round((min(수정시가) + max(수정시가)) / 2) 중간주가
//                 from     일봉
//                 where    단축코드 in ('000020', '000040')
//                 and 일자 between '20200101' and '20200303'
//                 group by 단축코드
//                 ) 기준
//         where   일봉.단축코드 in ('000020', '000040')
//           and   일자 between '20200101' and '20200303'
//           and   일봉.단축코드 = 기준.단축코드
//         group by 일자
//         )
// order by 일자          