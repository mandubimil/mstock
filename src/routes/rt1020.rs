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
    let (_, q_para, _) = util::body_to_hash_jo(body);


    let result_json = match id
    {        
        "get_ju_sang" =>
        {
            let query = "
            select   메인.단축코드,
                        종목명,
                        round((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전) / 6, 2) 합,       
                        round(일주일/한달전,2) 주_한달,
                        round(한달/한달전,2) 한_한달,
                        round(일주일/석달전,2) 주_석달,
                        round(한달/석달전,2) 한_석달,
                        round(일주일/한달전,2) 주_반년,
                        round(한달/한달전,2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 시가, 0)) 일주일,
                                sum(decode(구분, '한달', 시가, 0)) 한달,
                                sum(decode(구분, '한달전', 시가, 0)) 한달전,
                                sum(decode(구분, '석달전', 시가, 0)) 석달전,
                                sum(decode(구분, '반년전', 시가, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '석달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '반년전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                )
                        group by 단축코드
                        ) 메인,
                        종목마스터
            where    메인.단축코드 = 종목마스터.단축코드
                and    한달전<>0 and 석달전<>0 and 반년전<>0
                and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
                and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)  
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '합_' value 합,
                   '주_한달_' value 주_한달,
                   '한_한달_' value 한_한달,
                   '주_석달_' value 주_석달,
                   '한_석달_' value 한_석달,
                   '주_반년_' value 주_반년,
                   '한_반년_' value 한_반년
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_tm_ju_sang" =>
        {
            let query = "
            select   get_테마명(테마업종.코드) 테마명,
                    round( avg((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전)) / 6, 2) 합,
                    round( avg(일주일/한달전),2) 주_한달,
                    round( avg(한달/한달전),2) 한_한달,
                    round( avg(일주일/석달전),2) 주_석달,
                    round( avg(한달/석달전),2) 한_석달,
                    round( avg(일주일/한달전),2) 주_반년,
                    round( avg(한달/한달전),2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 시가, 0)) 일주일,
                                sum(decode(구분, '한달', 시가, 0)) 한달,
                                sum(decode(구분, '한달전', 시가, 0)) 한달전,
                                sum(decode(구분, '석달전', 시가, 0)) 석달전,
                                sum(decode(구분, '반년전', 시가, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '석달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '반년전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                )
                        group by 단축코드
                    ) 메인,
                    종목마스터,
                    테마업종
            where    메인.단축코드 = 종목마스터.단축코드
            and    한달전<>0 and 석달전<>0 and 반년전<>0
            and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
            and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)  
            and    테마업종.구분 = '테마내역'
            and    메인.단축코드 = 테마업종.내용
            group by get_테마명(테마업종.코드)
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '테마명' value 테마명,
                   '합_' value 합,
                   '주_한달_' value 주_한달,
                   '한_한달_' value 한_한달,
                   '주_석달_' value 주_석달,
                   '한_석달_' value 한_석달,
                   '주_반년_' value 주_반년,
                   '한_반년_' value 한_반년
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_uj_ju_sang" =>
        {
            let query = "
            select   get_업종명(테마업종.코드) 업종명,
                    round( avg((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전)) / 6, 2) 합,
                    round( avg(일주일/한달전),2) 주_한달,
                    round( avg(한달/한달전),2) 한_한달,
                    round( avg(일주일/석달전),2) 주_석달,
                    round( avg(한달/석달전),2) 한_석달,
                    round( avg(일주일/한달전),2) 주_반년,
                    round( avg(한달/한달전),2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 시가, 0)) 일주일,
                                sum(decode(구분, '한달', 시가, 0)) 한달,
                                sum(decode(구분, '한달전', 시가, 0)) 한달전,
                                sum(decode(구분, '석달전', 시가, 0)) 석달전,
                                sum(decode(구분, '반년전', 시가, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    group by 단축코드
                                    union   all
                                    select   '한달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '석달전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                    union   all
                                    select   '반년전' 구분, 단축코드, round(avg(수정시가)) 시가
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    group by 단축코드
                                )
                        group by 단축코드
                    ) 메인,
                    종목마스터,
                    테마업종
            where    메인.단축코드 = 종목마스터.단축코드
            and    한달전<>0 and 석달전<>0 and 반년전<>0
            and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
            and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)  
            and    테마업종.구분 = '업종내역'
            and    메인.단축코드 = 테마업종.내용
            group by get_업종명(테마업종.코드)
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '업종명' value 업종명,
                   '합_' value 합,
                   '주_한달_' value 주_한달,
                   '한_한달_' value 한_한달,
                   '주_석달_' value 주_석달,
                   '한_석달_' value 한_석달,
                   '주_반년_' value 주_반년,
                   '한_반년_' value 한_반년
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_ger_sang" =>
        {
            let query = "
            select   메인.단축코드,
                    종목명,
                    round((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전) / 6, 2) 합,
                    round(일주일/한달전,2) 주_한달,
                    round(한달/한달전,2) 한_한달,
                    round(일주일/석달전,2) 주_석달,
                    round(한달/석달전,2) 한_석달,
                    round(일주일/한달전,2) 주_반년,
                    round(한달/한달전,2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 거래량, 0)) 일주일,
                                sum(decode(구분, '한달', 거래량, 0)) 한달,
                                sum(decode(구분, '한달전', 거래량, 0)) 한달전,
                                sum(decode(구분, '석달전', 거래량, 0)) 석달전,
                                sum(decode(구분, '반년전', 거래량, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '석달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '반년전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                )
                        group by 단축코드
                    ) 메인,
                    종목마스터
            where    메인.단축코드 = 종목마스터.단축코드
            and    한달전<>0 and 석달전<>0 and 반년전<>0
            and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
            and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '단축코드' value 단축코드,
                   '종목명' value 종목명,
                   '합' value 합,
                   '주_한달' value 주_한달,
                   '한_한달' value 한_한달,
                   '주_석달' value 주_석달,
                   '한_석달' value 한_석달,
                   '주_반년' value 주_반년,
                   '한_반년' value 한_반년
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }        
        "get_tm_ger_sang" =>
        {
            let query = "
            select   get_테마명(테마업종.코드) 테마명,
                    round( avg((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전)) / 6, 2) 합,
                    round( avg(일주일/한달전),2) 주_한달,
                    round( avg(한달/한달전),2) 한_한달,
                    round( avg(일주일/석달전),2) 주_석달,
                    round( avg(한달/석달전),2) 한_석달,
                    round( avg(일주일/한달전),2) 주_반년,
                    round( avg(한달/한달전),2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 거래량, 0)) 일주일,
                                sum(decode(구분, '한달', 거래량, 0)) 한달,
                                sum(decode(구분, '한달전', 거래량, 0)) 한달전,
                                sum(decode(구분, '석달전', 거래량, 0)) 석달전,
                                sum(decode(구분, '반년전', 거래량, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '석달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '반년전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                )
                        group by 단축코드
                    ) 메인,
                    종목마스터,
                    테마업종
            where    메인.단축코드 = 종목마스터.단축코드
            and    한달전<>0 and 석달전<>0 and 반년전<>0
            and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
            and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)  
            and    테마업종.구분 = '테마내역'
            and    메인.단축코드 = 테마업종.내용
            group by get_테마명(테마업종.코드)
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '테마명' value 테마명,
                   '합_' value 합,
                   '주_한달_' value 주_한달,
                   '한_한달_' value 한_한달,
                   '주_석달_' value 주_석달,
                   '한_석달_' value 한_석달,
                   '주_반년_' value 주_반년,
                   '한_반년_' value 한_반년
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }    
        "get_uj_ger_sang" =>
        {
            let query = "
            select   get_업종명(테마업종.코드) 업종명,
                    round( avg((일주일/한달전 + 한달/한달전 + 일주일/석달전 + 한달/석달전 + 일주일/한달전 + 한달/한달전)) / 6, 2) 합,
                    round( avg(일주일/한달전),2) 주_한달,
                    round( avg(한달/한달전),2) 한_한달,
                    round( avg(일주일/석달전),2) 주_석달,
                    round( avg(한달/석달전),2) 한_석달,
                    round( avg(일주일/한달전),2) 주_반년,
                    round( avg(한달/한달전),2) 한_반년
            from     (
                        select   단축코드,
                                sum(decode(구분, '일주일', 거래량, 0)) 일주일,
                                sum(decode(구분, '한달', 거래량, 0)) 한달,
                                sum(decode(구분, '한달전', 거래량, 0)) 한달전,
                                sum(decode(구분, '석달전', 거래량, 0)) 석달전,
                                sum(decode(구분, '반년전', 거래량, 0)) 반년전
                        from     (
                                    select   '일주일' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(to_date( :종료일자, 'YYYYMMDD' )-7, 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD') and :종료일자
                                    union   all
                                    select   '한달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -2), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '석달전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -4), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                    union   all
                                    select   '반년전' 구분, 단축코드, 거래량
                                    from     일봉
                                    where  일자 between to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -7), 'YYYYMMDD') and to_char(add_months(to_date( :종료일자, 'YYYYMMDD' ), -1), 'YYYYMMDD')
                                )
                        group by 단축코드
                    ) 메인,
                    종목마스터,
                    테마업종
            where    메인.단축코드 = 종목마스터.단축코드
            and    한달전<>0 and 석달전<>0 and 반년전<>0
            and    종목마스터.구분 = decode(:시장, 'ZZ', 종목마스터.구분, :시장)
            and    종목마스터.etf구분 = decode(:etf, 'ZZ', 종목마스터.etf구분, :etf)  
            and    테마업종.구분 = '업종내역'
            and    메인.단축코드 = 테마업종.내용
            group by get_업종명(테마업종.코드)
            order by 합 desc
            ";

            let qs ="
            select json_object(
                   '업종명' value 업종명,
                   '합_' value 합,
                   '주_한달_' value 주_한달,
                   '한_한달_' value 한_한달,
                   '주_석달_' value 주_석달,
                   '한_석달_' value 한_석달,
                   '주_반년_' value 주_반년,
                   '한_반년_' value 한_반년
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



