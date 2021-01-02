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
    let (j_para, mut q_para, e_para) = util::body_to_hash_jo(body);

    let result_json = match id
    {        
        "get_dr_jong" =>
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
            select  단축코드, get_종목명(단축코드) 종목명, 일자, sum(거래량) 거래량,

                    round(avg(전일5고가/전일5시가*100), 2) 전5일_등락,
                    round(avg(전일4고가/전일4시가*100), 2) 전4일_등락,
                    round(avg(전일3고가/전일3시가*100), 2) 전3일_등락,
                    round(avg(전일2고가/전일2시가*100), 2) 전2일_등락,
                    round(avg(전일1고가/전일1시가*100), 2) 전1일_등락,
                    round(avg(수정고가/수정시가*100), 2) 당일_등락,
                    
                    round(avg(전일5시가/수정시가*100), 2) 전5일_시가,
                    round(avg(전일4시가/수정시가*100), 2) 전4일_시가,
                    round(avg(전일3시가/수정시가*100), 2) 전3일_시가,
                    round(avg(전일2시가/수정시가*100), 2) 전2일_시가,
                    round(avg(전일1시가/수정시가*100), 2) 전1일_시가,
                    100 당일_시가,
                    
                    round(avg(전일5종가/수정종가*100), 2) 전5일_종가,
                    round(avg(전일4종가/수정종가*100), 2) 전4일_종가,
                    round(avg(전일3종가/수정종가*100), 2) 전3일_종가,
                    round(avg(전일2종가/수정종가*100), 2) 전2일_종가,
                    round(avg(전일1종가/수정종가*100), 2) 전1일_종가,
                    100 당일_종가,
                    
                    round(avg(전일5고가/수정고가*100), 2) 전5일_고가,
                    round(avg(전일4고가/수정고가*100), 2) 전4일_고가,
                    round(avg(전일3고가/수정고가*100), 2) 전3일_고가,
                    round(avg(전일2고가/수정고가*100), 2) 전2일_고가,
                    round(avg(전일1고가/수정고가*100), 2) 전1일_고가,
                    100 당일_고가,
            
                    round(avg(전일5저가/수정저가*100), 2) 전5일_저가,
                    round(avg(전일4저가/수정저가*100), 2) 전4일_저가,
                    round(avg(전일3저가/수정저가*100), 2) 전3일_저가,
                    round(avg(전일2저가/수정저가*100), 2) 전2일_저가,
                    round(avg(전일1저가/수정저가*100), 2) 전1일_저가,
                    100 당일_저가
                    
            from    (
                    select   메인.단축코드,
                            일자,
                            수정시가,
                            수정종가,
                            수정고가,
                            수정저가,         
                            거래량,
                            LAG(수정시가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5시가,
                            LAG(수정시가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4시가,
                            LAG(수정시가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3시가,
                            LAG(수정시가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2시가,
                            LAG(수정시가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1시가,
                            LAG(수정종가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5종가,
                            LAG(수정종가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4종가,
                            LAG(수정종가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3종가,
                            LAG(수정종가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2종가,
                            LAG(수정종가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1종가,
                            LAG(수정고가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5고가,
                            LAG(수정고가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4고가,
                            LAG(수정고가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3고가,
                            LAG(수정고가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2고가,
                            LAG(수정고가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1고가,
                            LAG(수정저가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5저가,
                            LAG(수정저가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4저가,
                            LAG(수정저가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3저가,
                            LAG(수정저가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2저가,
                            LAG(수정저가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1저가            
                    from     일봉 메인, 종목마스터 종목
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                    and    메인.단축코드 = 종목.단축코드
                    and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                    and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    )
            where 일자 = :일자
            {} {}
            and 전일5시가<>0 and 전일4시가<>0 and 전일3시가<>0 and 전일2시가<>0 and 전일1시가<>0 and 수정시가<>0
            and 전일5종가<>0 and 전일4종가<>0 and 전일3종가<>0 and 전일2종가<>0 and 전일1종가<>0 and 수정종가<>0
            and 전일5고가<>0 and 전일4고가<>0 and 전일3고가<>0 and 전일2고가<>0 and 전일1고가<>0 and 수정고가<>0
            and 전일5저가<>0 and 전일4저가<>0 and 전일3저가<>0 and 전일2저가<>0 and 전일1저가<>0 and 수정저가<>0
            group by 단축코드, get_종목명(단축코드), 일자
            order by 단축코드, get_종목명(단축코드), 일자
            ", sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '단축코드' value 단축코드, 
                   '기준' value 종목명, 
                   '일자' value 일자, 
                   '거래량' value 거래량,

                   'a1' value '',
                   '전5일_등락' value 전5일_등락,
                   '전4일_등락' value 전4일_등락,
                   '전3일_등락' value 전3일_등락,
                   '전2일_등락' value 전2일_등락,
                   '전1일_등락' value 전1일_등락,
                   '당일_등락'  value 당일_등락,
                   
                   'a2' value '',
                   '전5일_시가' value 전5일_시가,
                   '전4일_시가' value 전4일_시가,
                   '전3일_시가' value 전3일_시가,
                   '전2일_시가' value 전2일_시가,
                   '전1일_시가' value 전1일_시가,
                   '당일_시가'  value 당일_시가,

                   'a3' value '',
                   '전5일_종가' value 전5일_종가,
                   '전4일_종가' value 전4일_종가,
                   '전3일_종가' value 전3일_종가,
                   '전2일_종가' value 전2일_종가,
                   '전1일_종가' value 전1일_종가,
                   '당일_종가'  value 당일_종가,

                   'a4' value '',
                   '전5일_고가' value 전5일_고가,
                   '전4일_고가' value 전4일_고가,
                   '전3일_고가' value 전3일_고가,
                   '전2일_고가' value 전2일_고가,
                   '전1일_고가' value 전1일_고가,
                   '당일_고가'  value 당일_고가,

                   'a5' value '',
                   '전5일_저가' value 전5일_저가,
                   '전4일_저가' value 전4일_저가,
                   '전3일_저가' value 전3일_저가,
                   '전2일_저가' value 전2일_저가,
                   '전1일_저가' value 전1일_저가,
                   '당일_저가'  value 당일_저가
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }
        "get_dr_si" =>
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
            select  시장, 일자, sum(거래량) 거래량,

                    round(avg(전일5고가/전일5시가*100), 2) 전5일_등락,
                    round(avg(전일4고가/전일4시가*100), 2) 전4일_등락,
                    round(avg(전일3고가/전일3시가*100), 2) 전3일_등락,
                    round(avg(전일2고가/전일2시가*100), 2) 전2일_등락,
                    round(avg(전일1고가/전일1시가*100), 2) 전1일_등락,
                    round(avg(수정고가/수정시가*100), 2) 당일_등락,
                    
                    round(avg(전일5시가/수정시가*100), 2) 전5일_시가,
                    round(avg(전일4시가/수정시가*100), 2) 전4일_시가,
                    round(avg(전일3시가/수정시가*100), 2) 전3일_시가,
                    round(avg(전일2시가/수정시가*100), 2) 전2일_시가,
                    round(avg(전일1시가/수정시가*100), 2) 전1일_시가,
                    100 당일_시가,
                    
                    round(avg(전일5종가/수정종가*100), 2) 전5일_종가,
                    round(avg(전일4종가/수정종가*100), 2) 전4일_종가,
                    round(avg(전일3종가/수정종가*100), 2) 전3일_종가,
                    round(avg(전일2종가/수정종가*100), 2) 전2일_종가,
                    round(avg(전일1종가/수정종가*100), 2) 전1일_종가,
                    100 당일_종가,
                    
                    round(avg(전일5고가/수정고가*100), 2) 전5일_고가,
                    round(avg(전일4고가/수정고가*100), 2) 전4일_고가,
                    round(avg(전일3고가/수정고가*100), 2) 전3일_고가,
                    round(avg(전일2고가/수정고가*100), 2) 전2일_고가,
                    round(avg(전일1고가/수정고가*100), 2) 전1일_고가,
                    100 당일_고가,
            
                    round(avg(전일5저가/수정저가*100), 2) 전5일_저가,
                    round(avg(전일4저가/수정저가*100), 2) 전4일_저가,
                    round(avg(전일3저가/수정저가*100), 2) 전3일_저가,
                    round(avg(전일2저가/수정저가*100), 2) 전2일_저가,
                    round(avg(전일1저가/수정저가*100), 2) 전1일_저가,
                    100 당일_저가
                    
            from    (
                    select  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드,
                            일자,
                            수정시가,
                            수정종가,
                            수정고가,
                            수정저가,         
                            거래량,
                            LAG(수정시가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5시가,
                            LAG(수정시가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4시가,
                            LAG(수정시가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3시가,
                            LAG(수정시가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2시가,
                            LAG(수정시가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1시가,
                            LAG(수정종가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5종가,
                            LAG(수정종가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4종가,
                            LAG(수정종가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3종가,
                            LAG(수정종가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2종가,
                            LAG(수정종가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1종가,
                            LAG(수정고가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5고가,
                            LAG(수정고가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4고가,
                            LAG(수정고가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3고가,
                            LAG(수정고가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2고가,
                            LAG(수정고가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1고가,
                            LAG(수정저가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5저가,
                            LAG(수정저가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4저가,
                            LAG(수정저가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3저가,
                            LAG(수정저가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2저가,
                            LAG(수정저가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1저가            
                    from     일봉 메인, 종목마스터 종목
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                    and    메인.단축코드 = 종목.단축코드
                    and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                    and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    )
            where 일자 = :일자
            {} {}
            and 전일5시가<>0 and 전일4시가<>0 and 전일3시가<>0 and 전일2시가<>0 and 전일1시가<>0 and 수정시가<>0
            and 전일5종가<>0 and 전일4종가<>0 and 전일3종가<>0 and 전일2종가<>0 and 전일1종가<>0 and 수정종가<>0
            and 전일5고가<>0 and 전일4고가<>0 and 전일3고가<>0 and 전일2고가<>0 and 전일1고가<>0 and 수정고가<>0
            and 전일5저가<>0 and 전일4저가<>0 and 전일3저가<>0 and 전일2저가<>0 and 전일1저가<>0 and 수정저가<>0
            group by 시장, 일자
            order by 시장, 일자
            ", sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '기준' value 시장, 
                   '일자' value 일자,
                   '거래량' value 거래량,

                   'a1' value '',
                   '전5일_등락' value 전5일_등락,
                   '전4일_등락' value 전4일_등락,
                   '전3일_등락' value 전3일_등락,
                   '전2일_등락' value 전2일_등락,
                   '전1일_등락' value 전1일_등락,
                   '당일_등락'  value 당일_등락,
                   
                   'a2' value '',
                   '전5일_시가' value 전5일_시가,
                   '전4일_시가' value 전4일_시가,
                   '전3일_시가' value 전3일_시가,
                   '전2일_시가' value 전2일_시가,
                   '전1일_시가' value 전1일_시가,
                   '당일_시가'  value 당일_시가,

                   'a3' value '',
                   '전5일_종가' value 전5일_종가,
                   '전4일_종가' value 전4일_종가,
                   '전3일_종가' value 전3일_종가,
                   '전2일_종가' value 전2일_종가,
                   '전1일_종가' value 전1일_종가,
                   '당일_종가'  value 당일_종가,

                   'a4' value '',
                   '전5일_고가' value 전5일_고가,
                   '전4일_고가' value 전4일_고가,
                   '전3일_고가' value 전3일_고가,
                   '전2일_고가' value 전2일_고가,
                   '전1일_고가' value 전1일_고가,
                   '당일_고가'  value 당일_고가,

                   'a5' value '',
                   '전5일_저가' value 전5일_저가,
                   '전4일_저가' value 전4일_저가,
                   '전3일_저가' value 전3일_저가,
                   '전2일_저가' value 전2일_저가,
                   '전1일_저가' value 전1일_저가,
                   '당일_저가'  value 당일_저가
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }
        "get_dr_uj" =>
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
            select  get_업종명(업종.코드) 업종, 일자, sum(거래량) 거래량,

                    round(avg(전일5고가/전일5시가*100), 2) 전5일_등락,
                    round(avg(전일4고가/전일4시가*100), 2) 전4일_등락,
                    round(avg(전일3고가/전일3시가*100), 2) 전3일_등락,
                    round(avg(전일2고가/전일2시가*100), 2) 전2일_등락,
                    round(avg(전일1고가/전일1시가*100), 2) 전1일_등락,
                    round(avg(수정고가/수정시가*100), 2) 당일_등락,
                    
                    round(avg(전일5시가/수정시가*100), 2) 전5일_시가,
                    round(avg(전일4시가/수정시가*100), 2) 전4일_시가,
                    round(avg(전일3시가/수정시가*100), 2) 전3일_시가,
                    round(avg(전일2시가/수정시가*100), 2) 전2일_시가,
                    round(avg(전일1시가/수정시가*100), 2) 전1일_시가,
                    100 당일_시가,
                    
                    round(avg(전일5종가/수정종가*100), 2) 전5일_종가,
                    round(avg(전일4종가/수정종가*100), 2) 전4일_종가,
                    round(avg(전일3종가/수정종가*100), 2) 전3일_종가,
                    round(avg(전일2종가/수정종가*100), 2) 전2일_종가,
                    round(avg(전일1종가/수정종가*100), 2) 전1일_종가,
                    100 당일_종가,
                    
                    round(avg(전일5고가/수정고가*100), 2) 전5일_고가,
                    round(avg(전일4고가/수정고가*100), 2) 전4일_고가,
                    round(avg(전일3고가/수정고가*100), 2) 전3일_고가,
                    round(avg(전일2고가/수정고가*100), 2) 전2일_고가,
                    round(avg(전일1고가/수정고가*100), 2) 전1일_고가,
                    100 당일_고가,
            
                    round(avg(전일5저가/수정저가*100), 2) 전5일_저가,
                    round(avg(전일4저가/수정저가*100), 2) 전4일_저가,
                    round(avg(전일3저가/수정저가*100), 2) 전3일_저가,
                    round(avg(전일2저가/수정저가*100), 2) 전2일_저가,
                    round(avg(전일1저가/수정저가*100), 2) 전1일_저가,
                    100 당일_저가
                    
            from    (
                    select  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드,
                            일자,
                            수정시가,
                            수정종가,
                            수정고가,
                            수정저가,         
                            거래량,
                            LAG(수정시가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5시가,
                            LAG(수정시가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4시가,
                            LAG(수정시가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3시가,
                            LAG(수정시가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2시가,
                            LAG(수정시가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1시가,
                            LAG(수정종가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5종가,
                            LAG(수정종가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4종가,
                            LAG(수정종가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3종가,
                            LAG(수정종가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2종가,
                            LAG(수정종가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1종가,
                            LAG(수정고가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5고가,
                            LAG(수정고가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4고가,
                            LAG(수정고가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3고가,
                            LAG(수정고가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2고가,
                            LAG(수정고가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1고가,
                            LAG(수정저가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5저가,
                            LAG(수정저가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4저가,
                            LAG(수정저가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3저가,
                            LAG(수정저가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2저가,
                            LAG(수정저가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1저가            
                    from     일봉 메인, 종목마스터 종목
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                    and    메인.단축코드 = 종목.단축코드
                    and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                    and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인,
                    (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '업종내역'
                    ) 업종                    
            where 일자 = :일자
              and 메인.단축코드 = 업종.내용
            {} {}
            and 전일5시가<>0 and 전일4시가<>0 and 전일3시가<>0 and 전일2시가<>0 and 전일1시가<>0 and 수정시가<>0
            and 전일5종가<>0 and 전일4종가<>0 and 전일3종가<>0 and 전일2종가<>0 and 전일1종가<>0 and 수정종가<>0
            and 전일5고가<>0 and 전일4고가<>0 and 전일3고가<>0 and 전일2고가<>0 and 전일1고가<>0 and 수정고가<>0
            and 전일5저가<>0 and 전일4저가<>0 and 전일3저가<>0 and 전일2저가<>0 and 전일1저가<>0 and 수정저가<>0
            group by get_업종명(업종.코드), 일자
            order by get_업종명(업종.코드), 일자
            ", sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '기준' value 업종, 
                   '일자' value 일자, 
                   '거래량' value 거래량,

                   'a1' value '',
                   '전5일_등락' value 전5일_등락,
                   '전4일_등락' value 전4일_등락,
                   '전3일_등락' value 전3일_등락,
                   '전2일_등락' value 전2일_등락,
                   '전1일_등락' value 전1일_등락,
                   '당일_등락'  value 당일_등락,
                   
                   'a2' value '',
                   '전5일_시가' value 전5일_시가,
                   '전4일_시가' value 전4일_시가,
                   '전3일_시가' value 전3일_시가,
                   '전2일_시가' value 전2일_시가,
                   '전1일_시가' value 전1일_시가,
                   '당일_시가'  value 당일_시가,

                   'a3' value '',
                   '전5일_종가' value 전5일_종가,
                   '전4일_종가' value 전4일_종가,
                   '전3일_종가' value 전3일_종가,
                   '전2일_종가' value 전2일_종가,
                   '전1일_종가' value 전1일_종가,
                   '당일_종가'  value 당일_종가,

                   'a4' value '',
                   '전5일_고가' value 전5일_고가,
                   '전4일_고가' value 전4일_고가,
                   '전3일_고가' value 전3일_고가,
                   '전2일_고가' value 전2일_고가,
                   '전1일_고가' value 전1일_고가,
                   '당일_고가'  value 당일_고가,

                   'a5' value '',
                   '전5일_저가' value 전5일_저가,
                   '전4일_저가' value 전4일_저가,
                   '전3일_저가' value 전3일_저가,
                   '전2일_저가' value 전2일_저가,
                   '전1일_저가' value 전1일_저가,
                   '당일_저가'  value 당일_저가
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }
        "get_dr_tm" =>
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
            select  get_테마명(테마.코드) 테마, 일자, sum(거래량) 거래량,

                    round(avg(전일5고가/전일5시가*100), 2) 전5일_등락,
                    round(avg(전일4고가/전일4시가*100), 2) 전4일_등락,
                    round(avg(전일3고가/전일3시가*100), 2) 전3일_등락,
                    round(avg(전일2고가/전일2시가*100), 2) 전2일_등락,
                    round(avg(전일1고가/전일1시가*100), 2) 전1일_등락,
                    round(avg(수정고가/수정시가*100), 2) 당일_등락,
                    
                    round(avg(전일5시가/수정시가*100), 2) 전5일_시가,
                    round(avg(전일4시가/수정시가*100), 2) 전4일_시가,
                    round(avg(전일3시가/수정시가*100), 2) 전3일_시가,
                    round(avg(전일2시가/수정시가*100), 2) 전2일_시가,
                    round(avg(전일1시가/수정시가*100), 2) 전1일_시가,
                    100 당일_시가,
                    
                    round(avg(전일5종가/수정종가*100), 2) 전5일_종가,
                    round(avg(전일4종가/수정종가*100), 2) 전4일_종가,
                    round(avg(전일3종가/수정종가*100), 2) 전3일_종가,
                    round(avg(전일2종가/수정종가*100), 2) 전2일_종가,
                    round(avg(전일1종가/수정종가*100), 2) 전1일_종가,
                    100 당일_종가,
                    
                    round(avg(전일5고가/수정고가*100), 2) 전5일_고가,
                    round(avg(전일4고가/수정고가*100), 2) 전4일_고가,
                    round(avg(전일3고가/수정고가*100), 2) 전3일_고가,
                    round(avg(전일2고가/수정고가*100), 2) 전2일_고가,
                    round(avg(전일1고가/수정고가*100), 2) 전1일_고가,
                    100 당일_고가,
            
                    round(avg(전일5저가/수정저가*100), 2) 전5일_저가,
                    round(avg(전일4저가/수정저가*100), 2) 전4일_저가,
                    round(avg(전일3저가/수정저가*100), 2) 전3일_저가,
                    round(avg(전일2저가/수정저가*100), 2) 전2일_저가,
                    round(avg(전일1저가/수정저가*100), 2) 전1일_저가,
                    100 당일_저가
                    
            from    (
                    select  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드,
                            일자,
                            수정시가,
                            수정종가,
                            수정고가,
                            수정저가,         
                            거래량,
                            LAG(수정시가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5시가,
                            LAG(수정시가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4시가,
                            LAG(수정시가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3시가,
                            LAG(수정시가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2시가,
                            LAG(수정시가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1시가,
                            LAG(수정종가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5종가,
                            LAG(수정종가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4종가,
                            LAG(수정종가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3종가,
                            LAG(수정종가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2종가,
                            LAG(수정종가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1종가,
                            LAG(수정고가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5고가,
                            LAG(수정고가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4고가,
                            LAG(수정고가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3고가,
                            LAG(수정고가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2고가,
                            LAG(수정고가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1고가,
                            LAG(수정저가,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일5저가,
                            LAG(수정저가,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일4저가,
                            LAG(수정저가,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일3저가,
                            LAG(수정저가,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일2저가,
                            LAG(수정저가,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 전일1저가            
                    from     일봉 메인, 종목마스터 종목
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                    and    메인.단축코드 = 종목.단축코드
                    and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                    and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인,
                    (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '테마내역'
                    ) 테마                    
            where 일자 = :일자
              and 메인.단축코드 = 테마.내용
            {} {}
            and 전일5시가<>0 and 전일4시가<>0 and 전일3시가<>0 and 전일2시가<>0 and 전일1시가<>0 and 수정시가<>0
            and 전일5종가<>0 and 전일4종가<>0 and 전일3종가<>0 and 전일2종가<>0 and 전일1종가<>0 and 수정종가<>0
            and 전일5고가<>0 and 전일4고가<>0 and 전일3고가<>0 and 전일2고가<>0 and 전일1고가<>0 and 수정고가<>0
            and 전일5저가<>0 and 전일4저가<>0 and 전일3저가<>0 and 전일2저가<>0 and 전일1저가<>0 and 수정저가<>0
            group by get_테마명(테마.코드), 일자
            order by get_테마명(테마.코드), 일자
            ", sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '기준' value 테마, 
                   '일자' value 일자, 
                   '거래량' value 거래량,

                   'a1' value '',
                   '전5일_등락' value 전5일_등락,
                   '전4일_등락' value 전4일_등락,
                   '전3일_등락' value 전3일_등락,
                   '전2일_등락' value 전2일_등락,
                   '전1일_등락' value 전1일_등락,
                   '당일_등락'  value 당일_등락,
                   
                   'a2' value '',
                   '전5일_시가' value 전5일_시가,
                   '전4일_시가' value 전4일_시가,
                   '전3일_시가' value 전3일_시가,
                   '전2일_시가' value 전2일_시가,
                   '전1일_시가' value 전1일_시가,
                   '당일_시가'  value 당일_시가,

                   'a3' value '',
                   '전5일_종가' value 전5일_종가,
                   '전4일_종가' value 전4일_종가,
                   '전3일_종가' value 전3일_종가,
                   '전2일_종가' value 전2일_종가,
                   '전1일_종가' value 전1일_종가,
                   '당일_종가'  value 당일_종가,

                   'a4' value '',
                   '전5일_고가' value 전5일_고가,
                   '전4일_고가' value 전4일_고가,
                   '전3일_고가' value 전3일_고가,
                   '전2일_고가' value 전2일_고가,
                   '전1일_고가' value 전1일_고가,
                   '당일_고가'  value 당일_고가,

                   'a5' value '',
                   '전5일_저가' value 전5일_저가,
                   '전4일_저가' value 전4일_저가,
                   '전3일_저가' value 전3일_저가,
                   '전2일_저가' value 전2일_저가,
                   '전1일_저가' value 전1일_저가,
                   '당일_저가'  value 당일_저가
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }
        "get_ger_jong" =>
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

            let mut np = String::new();
            if j_para["수량금액"] == "1".to_string()
            {
                np = "".to_string();
            }
            else if j_para["수량금액"] == "2".to_string()
            {
                np = "*종가".to_string();
            }

            let query = format!("
            SELECT  일자, 메인.단축코드, get_종목명(단축코드) 종목명,
                    SUM(외국계{}) 외국계,
                    SUM(기관{}) 기관,
                    SUM(개인{}) 개인,
                    SUM(프로그램{}) 프로그램,
                    SUM(외국계10{}+외국계9{}+외국계8{}+외국계7{}+외국계6{}+외국계5{}+외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계10합,
                    SUM(외국계5{} +외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계5합,
                    SUM(외국계5{}) 외국계5,
                    SUM(외국계4{}) 외국계4,
                    SUM(외국계3{}) 외국계3,
                    SUM(외국계2{}) 외국계2,
                    SUM(외국계1{}) 외국계1,
                    SUM(기관10{}+기관9{}+기관8{}+기관7{}+기관6{}+기관5{}+기관4{}+기관3{}+기관2{}+기관1{}) 기관10합,
                    SUM(기관5{} +기관4{}+기관3{}+기관2{}+기관1{}) 기관5합,
                    SUM(기관5{}) 기관5,
                    SUM(기관4{}) 기관4,
                    SUM(기관3{}) 기관3,
                    SUM(기관2{}) 기관2,
                    SUM(기관1{}) 기관1,
                    SUM(프로그램10{}+프로그램9{}+프로그램8{}+프로그램7{}+프로그램6{}+프로그램5{}+프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램10합,
                    SUM(프로그램5{} +프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램5합,
                    SUM(프로그램5{}) 프로그램5,
                    SUM(프로그램4{}) 프로그램4,
                    SUM(프로그램3{}) 프로그램3,
                    SUM(프로그램2{}) 프로그램2,
                    SUM(프로그램1{}) 프로그램1,
                    SUM(개인10{}+개인9{}+개인8{}+개인7{}+개인6{}+개인5{}+개인4{}+개인3{}+개인2{}+개인1{}) 개인10합,
                    SUM(개인5{} +개인4{}+개인3{}+개인2{}+개인1{}) 개인5합,
                    SUM(개인5{}) 개인5,
                    SUM(개인4{}) 개인4,
                    SUM(개인3{}) 개인3,
                    SUM(개인2{}) 개인2,
                    SUM(개인1{}) 개인1
                    
            FROM    (
                    SELECT  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드, 일자, 종가,                           
                            개인,
                            기관,
                            외국계,
                            프로그램,         
                            LAG(개인,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인10,                  
                            LAG(개인,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인9,                  
                            LAG(개인,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인8,                  
                            LAG(개인,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인7,                  
                            LAG(개인,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인6,
                            LAG(개인,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인5,                  
                            LAG(개인,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인4,                  
                            LAG(개인,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인3,                  
                            LAG(개인,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인2,                  
                            LAG(개인,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인1,
                            LAG(기관,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관10,                  
                            LAG(기관,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관9,                  
                            LAG(기관,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관8,                  
                            LAG(기관,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관7,                  
                            LAG(기관,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관6,
                            LAG(기관,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관5,                  
                            LAG(기관,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관4,                  
                            LAG(기관,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관3,                  
                            LAG(기관,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관2,                  
                            LAG(기관,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관1,
                            LAG(외국계,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계10,                  
                            LAG(외국계,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계9,                  
                            LAG(외국계,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계8,                  
                            LAG(외국계,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계7,                  
                            LAG(외국계,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계6,
                            LAG(외국계,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계5,                  
                            LAG(외국계,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계4,                  
                            LAG(외국계,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계3,                  
                            LAG(외국계,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계2,                  
                            LAG(외국계,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계1,
                            LAG(프로그램,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램10,                  
                            LAG(프로그램,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램9,                  
                            LAG(프로그램,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램8,                  
                            LAG(프로그램,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램7,                  
                            LAG(프로그램,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램6,
                            LAG(프로그램,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램5,                  
                            LAG(프로그램,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램4,                  
                            LAG(프로그램,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램3,                  
                            LAG(프로그램,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램2,                  
                            LAG(프로그램,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램1,
                            보유주식수,
                            누적거래량         
                    FROM    거래 메인, 종목마스터 
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                      AND   메인.단축코드 = 종목마스터.단축코드                    
                      and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                      and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인
            WHERE   일자 = :일자
            {} {}
            GROUP BY 일자, 메인.단축코드, get_종목명(단축코드)
            ORDER BY 일자, 메인.단축코드, get_종목명(단축코드)
            ", np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '단축코드' value 단축코드,
                   '기준' value 종목명,
                   '외국계' value 외국계,
                   '기관' value 기관,
                   '개인' value 개인,
                   '프로그램' value 프로그램,
                   'a1' value '',
                   '외국계10합' value 외국계10합,
                   '외국계5합' value 외국계5합,
                   'a11' value '',
                   '외국계5' value 외국계5,
                   '외국계4' value 외국계4,
                   '외국계3' value 외국계3,
                   '외국계2' value 외국계2,
                   '외국계1' value 외국계1,
                   'a2' value '',
                   '기관10합' value 기관10합,
                   '기관5합' value 기관5합,
                   'a21' value '',
                   '기관5' value 기관5,
                   '기관4' value 기관4,
                   '기관3' value 기관3,
                   '기관2' value 기관2,
                   '기관1' value 기관1,
                   'a3' value '',
                   '개인10합' value 개인10합,
                   '개인5합' value 개인5합,
                   'a31' value '',
                   '개인5' value 개인5,
                   '개인4' value 개인4,
                   '개인3' value 개인3,
                   '개인2' value 개인2,
                   '개인1' value 개인1,
                   'a4' value '',
                   '프로그램10합' value 프로그램10합,
                   '프로그램5합' value 프로그램5합,
                   'a41' value '',
                   '프로그램5' value 프로그램5,
                   '프로그램4' value 프로그램4,
                   '프로그램3' value 프로그램3,
                   '프로그램2' value 프로그램2,
                   '프로그램1' value 프로그램1
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }
        "get_ger_si" =>
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

            let mut np = String::new();
            if j_para["수량금액"] == "1".to_string()
            {
                np = "".to_string();
            }
            else if j_para["수량금액"] == "2".to_string()
            {
                np = "*종가".to_string();
            }

            let query = format!("
            SELECT  일자,시장,
                    SUM(외국계{}) 외국계,
                    SUM(기관{}) 기관,
                    SUM(개인{}) 개인,
                    SUM(프로그램{}) 프로그램,
                    SUM(외국계10{}+외국계9{}+외국계8{}+외국계7{}+외국계6{}+외국계5{}+외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계10합,
                    SUM(외국계5{} +외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계5합,
                    SUM(외국계5{}) 외국계5,
                    SUM(외국계4{}) 외국계4,
                    SUM(외국계3{}) 외국계3,
                    SUM(외국계2{}) 외국계2,
                    SUM(외국계1{}) 외국계1,
                    SUM(기관10{}+기관9{}+기관8{}+기관7{}+기관6{}+기관5{}+기관4{}+기관3{}+기관2{}+기관1{}) 기관10합,
                    SUM(기관5{} +기관4{}+기관3{}+기관2{}+기관1{}) 기관5합,
                    SUM(기관5{}) 기관5,
                    SUM(기관4{}) 기관4,
                    SUM(기관3{}) 기관3,
                    SUM(기관2{}) 기관2,
                    SUM(기관1{}) 기관1,
                    SUM(프로그램10{}+프로그램9{}+프로그램8{}+프로그램7{}+프로그램6{}+프로그램5{}+프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램10합,
                    SUM(프로그램5{} +프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램5합,
                    SUM(프로그램5{}) 프로그램5,
                    SUM(프로그램4{}) 프로그램4,
                    SUM(프로그램3{}) 프로그램3,
                    SUM(프로그램2{}) 프로그램2,
                    SUM(프로그램1{}) 프로그램1,
                    SUM(개인10{}+개인9{}+개인8{}+개인7{}+개인6{}+개인5{}+개인4{}+개인3{}+개인2{}+개인1{}) 개인10합,
                    SUM(개인5{} +개인4{}+개인3{}+개인2{}+개인1{}) 개인5합,
                    SUM(개인5{}) 개인5,
                    SUM(개인4{}) 개인4,
                    SUM(개인3{}) 개인3,
                    SUM(개인2{}) 개인2,
                    SUM(개인1{}) 개인1
                    
            FROM    (
                    SELECT  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드, 일자, 종가,                           
                            개인,
                            기관,
                            외국계,
                            프로그램,         
                            LAG(개인,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인10,                  
                            LAG(개인,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인9,                  
                            LAG(개인,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인8,                  
                            LAG(개인,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인7,                  
                            LAG(개인,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인6,
                            LAG(개인,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인5,                  
                            LAG(개인,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인4,                  
                            LAG(개인,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인3,                  
                            LAG(개인,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인2,                  
                            LAG(개인,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인1,
                            LAG(기관,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관10,                  
                            LAG(기관,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관9,                  
                            LAG(기관,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관8,                  
                            LAG(기관,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관7,                  
                            LAG(기관,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관6,
                            LAG(기관,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관5,                  
                            LAG(기관,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관4,                  
                            LAG(기관,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관3,                  
                            LAG(기관,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관2,                  
                            LAG(기관,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관1,
                            LAG(외국계,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계10,                  
                            LAG(외국계,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계9,                  
                            LAG(외국계,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계8,                  
                            LAG(외국계,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계7,                  
                            LAG(외국계,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계6,
                            LAG(외국계,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계5,                  
                            LAG(외국계,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계4,                  
                            LAG(외국계,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계3,                  
                            LAG(외국계,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계2,                  
                            LAG(외국계,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계1,
                            LAG(프로그램,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램10,                  
                            LAG(프로그램,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램9,                  
                            LAG(프로그램,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램8,                  
                            LAG(프로그램,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램7,                  
                            LAG(프로그램,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램6,
                            LAG(프로그램,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램5,                  
                            LAG(프로그램,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램4,                  
                            LAG(프로그램,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램3,                  
                            LAG(프로그램,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램2,                  
                            LAG(프로그램,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램1,
                            보유주식수,
                            누적거래량         
                    FROM    거래 메인, 종목마스터 
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                      AND   메인.단축코드 = 종목마스터.단축코드                    
                      and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                      and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인
            WHERE   일자 = :일자
            {} {}
            GROUP BY 일자,시장
            ORDER BY 일자,시장
            ", np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '기준' value 시장,
                   '외국계' value 외국계,
                   '기관' value 기관,
                   '개인' value 개인,
                   '프로그램' value 프로그램,
                   'a1' value '',
                   '외국계10합' value 외국계10합,
                   '외국계5합' value 외국계5합,
                   'a11' value '',
                   '외국계5' value 외국계5,
                   '외국계4' value 외국계4,
                   '외국계3' value 외국계3,
                   '외국계2' value 외국계2,
                   '외국계1' value 외국계1,
                   'a2' value '',
                   '기관10합' value 기관10합,
                   '기관5합' value 기관5합,
                   'a21' value '',
                   '기관5' value 기관5,
                   '기관4' value 기관4,
                   '기관3' value 기관3,
                   '기관2' value 기관2,
                   '기관1' value 기관1,
                   'a3' value '',
                   '개인10합' value 개인10합,
                   '개인5합' value 개인5합,
                   'a31' value '',
                   '개인5' value 개인5,
                   '개인4' value 개인4,
                   '개인3' value 개인3,
                   '개인2' value 개인2,
                   '개인1' value 개인1,
                   'a4' value '',
                   '프로그램10합' value 프로그램10합,
                   '프로그램5합' value 프로그램5합,
                   'a41' value '',
                   '프로그램5' value 프로그램5,
                   '프로그램4' value 프로그램4,
                   '프로그램3' value 프로그램3,
                   '프로그램2' value 프로그램2,
                   '프로그램1' value 프로그램1
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }        
        "get_ger_uj" =>
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

            let mut np = String::new();
            if j_para["수량금액"] == "1".to_string()
            {
                np = "".to_string();
            }
            else if j_para["수량금액"] == "2".to_string()
            {
                np = "*종가".to_string();
            }

            let query = format!("
            SELECT  일자,get_업종명(업종.코드) 업종,
                    SUM(외국계{}) 외국계,
                    SUM(기관{}) 기관,
                    SUM(개인{}) 개인,
                    SUM(프로그램{}) 프로그램,
                    SUM(외국계10{}+외국계9{}+외국계8{}+외국계7{}+외국계6{}+외국계5{}+외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계10합,
                    SUM(외국계5{} +외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계5합,
                    SUM(외국계5{}) 외국계5,
                    SUM(외국계4{}) 외국계4,
                    SUM(외국계3{}) 외국계3,
                    SUM(외국계2{}) 외국계2,
                    SUM(외국계1{}) 외국계1,
                    SUM(기관10{}+기관9{}+기관8{}+기관7{}+기관6{}+기관5{}+기관4{}+기관3{}+기관2{}+기관1{}) 기관10합,
                    SUM(기관5{} +기관4{}+기관3{}+기관2{}+기관1{}) 기관5합,
                    SUM(기관5{}) 기관5,
                    SUM(기관4{}) 기관4,
                    SUM(기관3{}) 기관3,
                    SUM(기관2{}) 기관2,
                    SUM(기관1{}) 기관1,
                    SUM(프로그램10{}+프로그램9{}+프로그램8{}+프로그램7{}+프로그램6{}+프로그램5{}+프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램10합,
                    SUM(프로그램5{} +프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램5합,
                    SUM(프로그램5{}) 프로그램5,
                    SUM(프로그램4{}) 프로그램4,
                    SUM(프로그램3{}) 프로그램3,
                    SUM(프로그램2{}) 프로그램2,
                    SUM(프로그램1{}) 프로그램1,
                    SUM(개인10{}+개인9{}+개인8{}+개인7{}+개인6{}+개인5{}+개인4{}+개인3{}+개인2{}+개인1{}) 개인10합,
                    SUM(개인5{} +개인4{}+개인3{}+개인2{}+개인1{}) 개인5합,
                    SUM(개인5{}) 개인5,
                    SUM(개인4{}) 개인4,
                    SUM(개인3{}) 개인3,
                    SUM(개인2{}) 개인2,
                    SUM(개인1{}) 개인1
                    
            FROM    (
                    SELECT  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드, 일자, 종가,                           
                            개인,
                            기관,
                            외국계,
                            프로그램,         
                            LAG(개인,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인10,                  
                            LAG(개인,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인9,                  
                            LAG(개인,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인8,                  
                            LAG(개인,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인7,                  
                            LAG(개인,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인6,
                            LAG(개인,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인5,                  
                            LAG(개인,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인4,                  
                            LAG(개인,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인3,                  
                            LAG(개인,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인2,                  
                            LAG(개인,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인1,
                            LAG(기관,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관10,                  
                            LAG(기관,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관9,                  
                            LAG(기관,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관8,                  
                            LAG(기관,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관7,                  
                            LAG(기관,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관6,
                            LAG(기관,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관5,                  
                            LAG(기관,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관4,                  
                            LAG(기관,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관3,                  
                            LAG(기관,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관2,                  
                            LAG(기관,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관1,
                            LAG(외국계,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계10,                  
                            LAG(외국계,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계9,                  
                            LAG(외국계,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계8,                  
                            LAG(외국계,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계7,                  
                            LAG(외국계,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계6,
                            LAG(외국계,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계5,                  
                            LAG(외국계,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계4,                  
                            LAG(외국계,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계3,                  
                            LAG(외국계,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계2,                  
                            LAG(외국계,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계1,
                            LAG(프로그램,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램10,                  
                            LAG(프로그램,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램9,                  
                            LAG(프로그램,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램8,                  
                            LAG(프로그램,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램7,                  
                            LAG(프로그램,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램6,
                            LAG(프로그램,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램5,                  
                            LAG(프로그램,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램4,                  
                            LAG(프로그램,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램3,                  
                            LAG(프로그램,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램2,                  
                            LAG(프로그램,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램1,
                            보유주식수,
                            누적거래량         
                    FROM    거래 메인, 종목마스터 
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                      AND   메인.단축코드 = 종목마스터.단축코드                    
                      and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                      and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인,
                    (
                        select   코드, 내용
                        from    테마업종
                        where  구분 = '업종내역'
                    ) 업종                    
            WHERE   일자 = :일자
              and   메인.단축코드 = 업종.내용
            {} {}
            GROUP BY 일자,get_업종명(업종.코드)
            ORDER BY 일자,get_업종명(업종.코드)
            ", np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '기준' value 업종,
                   '외국계' value 외국계,
                   '기관' value 기관,
                   '개인' value 개인,
                   '프로그램' value 프로그램,
                   'a1' value '',
                   '외국계10합' value 외국계10합,
                   '외국계5합' value 외국계5합,
                   'a11' value '',
                   '외국계5' value 외국계5,
                   '외국계4' value 외국계4,
                   '외국계3' value 외국계3,
                   '외국계2' value 외국계2,
                   '외국계1' value 외국계1,
                   'a2' value '',
                   '기관10합' value 기관10합,
                   '기관5합' value 기관5합,
                   'a21' value '',
                   '기관5' value 기관5,
                   '기관4' value 기관4,
                   '기관3' value 기관3,
                   '기관2' value 기관2,
                   '기관1' value 기관1,
                   'a3' value '',
                   '개인10합' value 개인10합,
                   '개인5합' value 개인5합,
                   'a31' value '',
                   '개인5' value 개인5,
                   '개인4' value 개인4,
                   '개인3' value 개인3,
                   '개인2' value 개인2,
                   '개인1' value 개인1,
                   'a4' value '',
                   '프로그램10합' value 프로그램10합,
                   '프로그램5합' value 프로그램5합,
                   'a41' value '',
                   '프로그램5' value 프로그램5,
                   '프로그램4' value 프로그램4,
                   '프로그램3' value 프로그램3,
                   '프로그램2' value 프로그램2,
                   '프로그램1' value 프로그램1
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
            db::select_json(db, &query_tot, q_para)
        }   
        "get_ger_tm" =>
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

            let mut np = String::new();
            if j_para["수량금액"] == "1".to_string()
            {
                np = "".to_string();
            }
            else if j_para["수량금액"] == "2".to_string()
            {
                np = "*종가".to_string();
            }

            let query = format!("
            SELECT  일자,get_테마명(테마.코드) 테마,
                    SUM(외국계{}) 외국계,
                    SUM(기관{}) 기관,
                    SUM(개인{}) 개인,
                    SUM(프로그램{}) 프로그램,
                    SUM(외국계10{}+외국계9{}+외국계8{}+외국계7{}+외국계6{}+외국계5{}+외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계10합,
                    SUM(외국계5{} +외국계4{}+외국계3{}+외국계2{}+외국계1{}) 외국계5합,
                    SUM(외국계5{}) 외국계5,
                    SUM(외국계4{}) 외국계4,
                    SUM(외국계3{}) 외국계3,
                    SUM(외국계2{}) 외국계2,
                    SUM(외국계1{}) 외국계1,
                    SUM(기관10{}+기관9{}+기관8{}+기관7{}+기관6{}+기관5{}+기관4{}+기관3{}+기관2{}+기관1{}) 기관10합,
                    SUM(기관5{} +기관4{}+기관3{}+기관2{}+기관1{}) 기관5합,
                    SUM(기관5{}) 기관5,
                    SUM(기관4{}) 기관4,
                    SUM(기관3{}) 기관3,
                    SUM(기관2{}) 기관2,
                    SUM(기관1{}) 기관1,
                    SUM(프로그램10{}+프로그램9{}+프로그램8{}+프로그램7{}+프로그램6{}+프로그램5{}+프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램10합,
                    SUM(프로그램5{} +프로그램4{}+프로그램3{}+프로그램2{}+프로그램1{}) 프로그램5합,
                    SUM(프로그램5{}) 프로그램5,
                    SUM(프로그램4{}) 프로그램4,
                    SUM(프로그램3{}) 프로그램3,
                    SUM(프로그램2{}) 프로그램2,
                    SUM(프로그램1{}) 프로그램1,
                    SUM(개인10{}+개인9{}+개인8{}+개인7{}+개인6{}+개인5{}+개인4{}+개인3{}+개인2{}+개인1{}) 개인10합,
                    SUM(개인5{} +개인4{}+개인3{}+개인2{}+개인1{}) 개인5합,
                    SUM(개인5{}) 개인5,
                    SUM(개인4{}) 개인4,
                    SUM(개인3{}) 개인3,
                    SUM(개인2{}) 개인2,
                    SUM(개인1{}) 개인1
                    
            FROM    (
                    SELECT  decode(etf구분, '0', '일반', '1', 'ETF', '2', 'ETN', '?') 시장,
                            메인.단축코드, 일자, 종가,                           
                            개인,
                            기관,
                            외국계,
                            프로그램,         
                            LAG(개인,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인10,                  
                            LAG(개인,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인9,                  
                            LAG(개인,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인8,                  
                            LAG(개인,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인7,                  
                            LAG(개인,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인6,
                            LAG(개인,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인5,                  
                            LAG(개인,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인4,                  
                            LAG(개인,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인3,                  
                            LAG(개인,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인2,                  
                            LAG(개인,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 개인1,
                            LAG(기관,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관10,                  
                            LAG(기관,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관9,                  
                            LAG(기관,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관8,                  
                            LAG(기관,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관7,                  
                            LAG(기관,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관6,
                            LAG(기관,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관5,                  
                            LAG(기관,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관4,                  
                            LAG(기관,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관3,                  
                            LAG(기관,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관2,                  
                            LAG(기관,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 기관1,
                            LAG(외국계,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계10,                  
                            LAG(외국계,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계9,                  
                            LAG(외국계,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계8,                  
                            LAG(외국계,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계7,                  
                            LAG(외국계,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계6,
                            LAG(외국계,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계5,                  
                            LAG(외국계,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계4,                  
                            LAG(외국계,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계3,                  
                            LAG(외국계,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계2,                  
                            LAG(외국계,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 외국계1,
                            LAG(프로그램,10,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램10,                  
                            LAG(프로그램,9,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램9,                  
                            LAG(프로그램,8,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램8,                  
                            LAG(프로그램,7,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램7,                  
                            LAG(프로그램,6,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램6,
                            LAG(프로그램,5,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램5,                  
                            LAG(프로그램,4,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램4,                  
                            LAG(프로그램,3,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램3,                  
                            LAG(프로그램,2,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램2,                  
                            LAG(프로그램,1,0) OVER (PARTITION BY 메인.단축코드 ORDER BY 메인.단축코드, 일자) 프로그램1,
                            보유주식수,
                            누적거래량         
                    FROM    거래 메인, 종목마스터 
                    WHERE   일자 BETWEEN  TO_CHAR(TO_DATE(:일자, 'YYYYMMDD')-30, 'YYYYMMDD') AND :일자
                      AND   메인.단축코드 = 종목마스터.단축코드                    
                      and   구분 = decode(:시장, 'ZZ', 구분, :시장)
                      and   etf구분 = decode(:etf, 'ZZ', etf구분, :etf)        
                    ) 메인,
                    (
                        select 코드, 내용
                        from   테마업종
                        where  구분 = '테마내역'
                    ) 테마                    
            WHERE   일자 = :일자
              and   메인.단축코드 = 테마.내용
            {} {}
            GROUP BY 일자,get_테마명(테마.코드)
            ORDER BY 일자,get_테마명(테마.코드)
            ", np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,   np,np,np,np,np,
               sql_jo_1, sql_jo_2);

            let qs ="
            select json_object(
                   '일자' value 일자,
                   '기준' value 테마,
                   '외국계' value 외국계,
                   '기관' value 기관,
                   '개인' value 개인,
                   '프로그램' value 프로그램,
                   'a1' value '',
                   '외국계10합' value 외국계10합,
                   '외국계5합' value 외국계5합,
                   'a11' value '',
                   '외국계5' value 외국계5,
                   '외국계4' value 외국계4,
                   '외국계3' value 외국계3,
                   '외국계2' value 외국계2,
                   '외국계1' value 외국계1,
                   'a2' value '',
                   '기관10합' value 기관10합,
                   '기관5합' value 기관5합,
                   'a21' value '',
                   '기관5' value 기관5,
                   '기관4' value 기관4,
                   '기관3' value 기관3,
                   '기관2' value 기관2,
                   '기관1' value 기관1,
                   'a3' value '',
                   '개인10합' value 개인10합,
                   '개인5합' value 개인5합,
                   'a31' value '',
                   '개인5' value 개인5,
                   '개인4' value 개인4,
                   '개인3' value 개인3,
                   '개인2' value 개인2,
                   '개인1' value 개인1,
                   'a4' value '',
                   '프로그램10합' value 프로그램10합,
                   '프로그램5합' value 프로그램5합,
                   'a41' value '',
                   '프로그램5' value 프로그램5,
                   '프로그램4' value 프로그램4,
                   '프로그램3' value 프로그램3,
                   '프로그램2' value 프로그램2,
                   '프로그램1' value 프로그램1
                    )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);

            //println!("{}", query_tot);
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