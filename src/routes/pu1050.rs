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
    let (_, para, _) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_chart" =>
        {
            let query ="
            select   일자,
                     종가 주가,
                     누적거래량,
                     개인,
                     기관,
                     외국인,
                     외국계,
                     프로그램,
                     SUM(SUM(외국계)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국계_누적,
                     SUM(SUM(외국인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 외국인_누적,
                     SUM(SUM(기관)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 기관_누적,
                     SUM(SUM(개인)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 개인_누적,
                     SUM(SUM(프로그램)) OVER(ORDER BY 일자 ASC ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) 프로그램_누적,
                     decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위
            from     거래
            where    단축코드 = :단축코드 and 일자 between :시작일자 and :종료일자
            group by 일자, 종가, 누적거래량, 개인, 기관, 외국인, 외국계, 프로그램
            order by 일자
            ";

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '주가' value 주가,
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
        "get_chart_basic" =>
        {
            let query ="
            select   max(일봉.단축코드) 단축코드,
                     max(종목명) 종목명,
                     max(시가총액) 시가총액, 
                     max(상장주식) 상장주식,
                     max(per) per, max(pbr) pbr,
                     max(최고52주) 최고52주, max(최저52주) 최저52주,
                     max(외국인보유) 외국인보유,
                     min(수정시가) 최저주가, max(수정시가) 최고주가, round(avg(수정시가)) 평균주가,
                     min(일자) 최저일자, max(일자) 최고일자, count(일자) 일자수, 
                     round(avg(거래량)) 평균거래, max(거래량) 최고거래
            from     일봉, 종목마스터
            where    일봉.단축코드 = :단축코드 
              and    일자 between :시작일자 and :종료일자
              and    일봉.단축코드 = 종목마스터.단축코드
            order by 일자
            ";

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '시가총액' value 시가총액,
                   '상장주식' value 상장주식,
                   'per' value per, 'pbr' value pbr,
                   '최고52주' value 최고52주, '최저52주' value 최저52주,
                   '외국인보유' value 외국인보유,
                   '최저주가' value 최저주가, '최고주가' value 최고주가, '평균주가' value 평균주가,
                   '최저일자' value 최저일자, '최고일자' value 최고일자, '일자수' value 일자수,
                   '평균거래' value 평균거래, '최고거래' value 최고거래                                      
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