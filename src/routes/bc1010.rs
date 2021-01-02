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
    let (_, mut para, para_gita) = util::body_to_hash_jo(body);

    let result_json = match id
    {
        "get_chart" =>
        {
            let query ="
            select 일자,
                    round(수정종가) 주가,
                    거래량,
                    decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위,
                    round(
                        (
                                수정종가+
                                lag(수정종가,1,0) over(order by 일자) + lag(수정종가,2,0) over(order by 일자)+
                                lag(수정종가,3,0) over(order by 일자) + lag(수정종가,4,0) over(order by 일자)+
                                lag(수정종가,5,0) over(order by 일자) + lag(수정종가,6,0) over(order by 일자)+
                                lag(수정종가,7,0) over(order by 일자) + lag(수정종가,8,0) over(order by 일자)+
                                lag(수정종가,9,0) over(order by 일자) + lag(수정종가,10,0) over(order by 일자)+
                                lag(수정종가,11,0) over(order by 일자) + lag(수정종가,12,0) over(order by 일자)+
                                lag(수정종가,13,0) over(order by 일자) + lag(수정종가,14,0) over(order by 일자)+
                                lag(수정종가,15,0) over(order by 일자) + lag(수정종가,16,0) over(order by 일자)+
                                lag(수정종가,17,0) over(order by 일자) + lag(수정종가,18,0) over(order by 일자)+
                                lag(수정종가,19,0) over(order by 일자) + lag(수정종가,20,0) over(order by 일자)
                        )
                        /
                        (
                            1+
                            decode(lag(수정종가,1,0) over(order by 일자),0,0,1)+decode(lag(수정종가,2,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,3,0) over(order by 일자),0,0,1)+decode(lag(수정종가,4,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,5,0) over(order by 일자),0,0,1)+decode(lag(수정종가,6,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,7,0) over(order by 일자),0,0,1)+decode(lag(수정종가,8,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,9,0) over(order by 일자),0,0,1)+decode(lag(수정종가,10,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,11,0) over(order by 일자),0,0,1)+decode(lag(수정종가,12,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,13,0) over(order by 일자),0,0,1)+decode(lag(수정종가,14,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,15,0) over(order by 일자),0,0,1)+decode(lag(수정종가,16,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,17,0) over(order by 일자),0,0,1)+decode(lag(수정종가,18,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,19,0) over(order by 일자),0,0,1)+decode(lag(수정종가,20,0) over(order by 일자),0,0,1)
                            )
                        ) 주가30일,
                round(
                        (
                                수정종가+
                                lag(수정종가,1,0) over(order by 일자) + lag(수정종가,2,0) over(order by 일자)+
                                lag(수정종가,3,0) over(order by 일자) + lag(수정종가,4,0) over(order by 일자)+
                                lag(수정종가,5,0) over(order by 일자) + lag(수정종가,6,0) over(order by 일자)+
                                lag(수정종가,7,0) over(order by 일자) + lag(수정종가,8,0) over(order by 일자)+
                                lag(수정종가,9,0) over(order by 일자) + lag(수정종가,10,0) over(order by 일자)+
                                lag(수정종가,11,0) over(order by 일자) + lag(수정종가,12,0) over(order by 일자)+
                                lag(수정종가,13,0) over(order by 일자) + lag(수정종가,14,0) over(order by 일자)+
                                lag(수정종가,15,0) over(order by 일자) + lag(수정종가,16,0) over(order by 일자)+
                                lag(수정종가,17,0) over(order by 일자) + lag(수정종가,18,0) over(order by 일자)+
                                lag(수정종가,19,0) over(order by 일자) + lag(수정종가,20,0) over(order by 일자)+
                                lag(수정종가,21,0) over(order by 일자) + lag(수정종가,22,0) over(order by 일자)+
                                lag(수정종가,23,0) over(order by 일자) + lag(수정종가,24,0) over(order by 일자)+
                                lag(수정종가,25,0) over(order by 일자) + lag(수정종가,26,0) over(order by 일자)+
                                lag(수정종가,27,0) over(order by 일자) + lag(수정종가,28,0) over(order by 일자)+
                                lag(수정종가,29,0) over(order by 일자) + lag(수정종가,30,0) over(order by 일자)+
                                lag(수정종가,31,0) over(order by 일자) + lag(수정종가,32,0) over(order by 일자)+
                                lag(수정종가,33,0) over(order by 일자) + lag(수정종가,34,0) over(order by 일자)+
                                lag(수정종가,35,0) over(order by 일자) + lag(수정종가,36,0) over(order by 일자)+
                                lag(수정종가,37,0) over(order by 일자) + lag(수정종가,38,0) over(order by 일자)+
                                lag(수정종가,39,0) over(order by 일자) + lag(수정종가,40,0) over(order by 일자)+
                                lag(수정종가,41,0) over(order by 일자) + lag(수정종가,42,0) over(order by 일자)+
                                lag(수정종가,43,0) over(order by 일자) + lag(수정종가,44,0) over(order by 일자)+
                                lag(수정종가,45,0) over(order by 일자) + lag(수정종가,46,0) over(order by 일자)+
                                lag(수정종가,47,0) over(order by 일자) + lag(수정종가,48,0) over(order by 일자)+
                                lag(수정종가,49,0) over(order by 일자) + lag(수정종가,50,0) over(order by 일자)+
                                lag(수정종가,51,0) over(order by 일자) + lag(수정종가,52,0) over(order by 일자)+
                                lag(수정종가,53,0) over(order by 일자) + lag(수정종가,54,0) over(order by 일자)+
                                lag(수정종가,55,0) over(order by 일자) + lag(수정종가,56,0) over(order by 일자)+
                                lag(수정종가,57,0) over(order by 일자) + lag(수정종가,58,0) over(order by 일자)+
                                lag(수정종가,59,0) over(order by 일자) + lag(수정종가,60,0) over(order by 일자)
                        )
                        /
                        (
                            1+
                            decode(lag(수정종가,1,0) over(order by 일자),0,0,1)+decode(lag(수정종가,2,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,3,0) over(order by 일자),0,0,1)+decode(lag(수정종가,4,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,5,0) over(order by 일자),0,0,1)+decode(lag(수정종가,6,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,7,0) over(order by 일자),0,0,1)+decode(lag(수정종가,8,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,9,0) over(order by 일자),0,0,1)+decode(lag(수정종가,10,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,11,0) over(order by 일자),0,0,1)+decode(lag(수정종가,12,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,13,0) over(order by 일자),0,0,1)+decode(lag(수정종가,14,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,15,0) over(order by 일자),0,0,1)+decode(lag(수정종가,16,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,17,0) over(order by 일자),0,0,1)+decode(lag(수정종가,18,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,19,0) over(order by 일자),0,0,1)+decode(lag(수정종가,20,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,21,0) over(order by 일자),0,0,1)+decode(lag(수정종가,22,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,23,0) over(order by 일자),0,0,1)+decode(lag(수정종가,24,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,25,0) over(order by 일자),0,0,1)+decode(lag(수정종가,26,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,27,0) over(order by 일자),0,0,1)+decode(lag(수정종가,28,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,29,0) over(order by 일자),0,0,1)+decode(lag(수정종가,30,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,31,0) over(order by 일자),0,0,1)+decode(lag(수정종가,32,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,33,0) over(order by 일자),0,0,1)+decode(lag(수정종가,34,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,35,0) over(order by 일자),0,0,1)+decode(lag(수정종가,36,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,37,0) over(order by 일자),0,0,1)+decode(lag(수정종가,38,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,39,0) over(order by 일자),0,0,1)+decode(lag(수정종가,40,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,41,0) over(order by 일자),0,0,1)+decode(lag(수정종가,42,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,43,0) over(order by 일자),0,0,1)+decode(lag(수정종가,44,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,45,0) over(order by 일자),0,0,1)+decode(lag(수정종가,46,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,47,0) over(order by 일자),0,0,1)+decode(lag(수정종가,48,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,49,0) over(order by 일자),0,0,1)+decode(lag(수정종가,50,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,51,0) over(order by 일자),0,0,1)+decode(lag(수정종가,52,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,53,0) over(order by 일자),0,0,1)+decode(lag(수정종가,54,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,55,0) over(order by 일자),0,0,1)+decode(lag(수정종가,56,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,57,0) over(order by 일자),0,0,1)+decode(lag(수정종가,58,0) over(order by 일자),0,0,1)+
                            decode(lag(수정종가,59,0) over(order by 일자),0,0,1)+decode(lag(수정종가,60,0) over(order by 일자),0,0,1)
                            )
                        ) 주가60일
            from 일봉
            where 단축코드 = :단축코드 and 일자 between :시작일자 and :종료일자
            order by 일자
            ";


            let qs ="
            select json_object(
                   '일자' value 일자,
                   '주가' value 주가,
                   '주가30일' value 주가30일,
                   '주가60일' value 주가60일,
                   '거래량' value 거래량,
                   '월단위' value 월단위
                   )
            from   (";
            let qe ="
                )";
            
            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, para)
        }
        "get_chart_select" =>
        {
            let query ="
            select   일자,
                     round((수정종가 / 중간주가 ) * 1000) 주가,
                     decode(substr(일자,3,4), substr(lag(일자,1,0) over(order by 일자),3,4), ' ', substr(일자,3,4)) 월단위,
                     중간주가
            from     일봉,
                     (
                     select   round((min(수정종가) + max(수정종가)) / 2) 중간주가
                     from     일봉
                     where    단축코드 = :단축코드 and 일자 between :시작일자 and :종료일자
                     )
            where    단축코드 = :단축코드 and 일자 between :시작일자 and :종료일자
            order by 일자
            ";

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
            let query ="
            select   max(일봉.단축코드) 단축코드,
                     max(종목명) 종목명,
                     max(시가총액) 시가총액, 
                     max(상장주식) 상장주식,
                     max(per) per, max(pbr) pbr,
                     max(최고52주) 최고52주, max(최저52주) 최저52주,
                     max(외국인보유) 외국인보유,
                     min(수정종가) 최저주가, max(수정종가) 최고주가, round(avg(수정종가)) 평균주가,
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
        "get_jong_list" => 
        {
            let mut sql_jo_1 = String::new();
            if para_gita["테마"] != "ZZ"
            {
                sql_jo_1 = " and 단축코드 in (select 내용 from 테마업종 where 구분='테마내역' and 코드 = :테마)".to_string();
                para.insert("테마".to_string(), para_gita["테마"].clone());
            }
 
            let mut sql_jo_2 = String::new();
            if para_gita["업종"] != "ZZ"
            {
                sql_jo_2 = " and 단축코드 in (select 내용 from 테마업종 where 구분='업종내역' and 코드 = :업종)".to_string();
                para.insert("업종".to_string(), para_gita["업종"].clone());
            }

            let query = "
            SELECT   JSON_OBJECT('단축코드' VALUE 단축코드, '종목명' VALUE 종목명) JSON
            FROM     종목마스터
            where    구분 = decode(:시장, 'ZZ', 구분, :시장)
              and    etf구분 = decode(:etf, 'ZZ', etf구분, :etf)
            ";

            let query_2 = format!("{} {} {} ", query, sql_jo_1, sql_jo_2);
            
            db::select_json(db, &query_2, para)
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