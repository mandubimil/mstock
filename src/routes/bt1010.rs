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
    let (_, q_para, e_para) = util::body_to_hash_jo(body);


    let result_json = match id
    {        
        "get_medo_list_detail" =>
        {
            let query = format!("
                        select  비교일자, 주기준,          
                                확률,          
                                건수,          
                                상승기준,          
                                상승판매,          
                                하락판매,          
                                보유일,          
                                count(단축코드) 매매건수,          
                                sum(매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015)) 최종차액,          
                                sum(매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015))/count(단축코드) 건당금액,          
                                sum(decode(매도구분, '상승판매', 1, 0)+decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), 1, 1, 0), 0))/count(단축코드) * 100 승률,          
                                sum(decode(매도구분, '상승판매', 1, 0)) 승,          
                                sum(decode(매도구분, '하락판매', 1, 0)) 패,          
                                sum(decode(매도구분, '기간만료', 1, 0)) 만료,          
                                sum(decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), 1, 1, 0), 0)) 만료승,          
                                sum(decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), -1, 1, 0), 0)) 만료패,          
                                sum(매수단가*매수수량*0.0015+매도단가*매도수량*0.0015) 세금,          
                                sum(매도단가*매도수량 - 매수단가*매수수량) 차액,
                                sum(매수수량*매수단가) 매수금액,          
                                sum(매도수량*매도단가) 매도금액          
                        from     {}          
                        where      매도단가 is not null   
                            and    비교일자 = decode(:비교일자1, '', 비교일자, :비교일자2)          
                            and    주기준 = decode(:주기준1, '', 주기준, :주기준2)          
                            and    확률 = decode(:확률1, '', 확률, :확률2)          
                            and    건수 = decode(:건수1, '', 건수, :건수2)          
                            and    상승기준 = decode(:상승기준1, '', 상승기준, :상승기준2)          
                            and    상승판매 = decode(:상승판매1, '', 상승판매, :상승판매2)          
                            and    하락판매 = decode(:하락판매1, '', 하락판매, :하락판매2)          
                            and    보유일 = decode(:보유일1, '', 보유일, :보유일2)          
                            and    (매도단가/매수단가) < 2
                        group by 비교일자, 주기준,          
                                확률,          
                                건수,          
                                상승기준,          
                                상승판매,          
                                하락판매,          
                                보유일
                        order by sum(매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015))/count(단축코드) desc

            ", e_para["테이블이름"]);                   

            let qs ="
            select json_object(
                   '비교일자' value 비교일자,
                   '주기준' value 주기준,
                   '확률' value 확률,
                   '건수' value 건수,
                   '상승기준' value 상승기준,
                   '상승판매' value 상승판매,
                   '하락판매' value 하락판매,
                   '보유일' value 보유일,
                   '매매건수_' value 매매건수,
                   '최종차액_' value 최종차액,
                   '건당금액_' value 건당금액,
                   '승률' value 승률,
                   '승' value 승,
                   '패' value 패,
                   '만료' value 만료,
                   '만료' value 만료승,
                   '만료패' value 만료패,
                   '세금' value 세금,
                   '차액' value 차액,
                   '매수금액' value 매수금액,
                   '매도금액' value 매도금액
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }
        "get_medo_list_detail_hadan" =>
        {
            let query = format!("
            select 메인.* from (          
                        select   비교일자, 주기준,          
                                확률,          
                                건수,          
                                상승기준,          
                                상승판매,          
                                하락판매,          
                                보유일,          
                                count(단축코드) 매매건수,          
                                sum(매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015)) 최종차액,          
                                sum(매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015))/count(단축코드) 건당금액,          
                                sum(decode(매도구분, '상승판매', 1, 0)+decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), 1, 1, 0), 0))/count(단축코드) * 100 승률,          
                                sum(decode(매도구분, '상승판매', 1, 0)) 승,          
                                sum(decode(매도구분, '하락판매', 1, 0)) 패,          
                                sum(decode(매도구분, '기간만료', 1, 0)) 만료,          
                                sum(decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), 1, 1, 0), 0)) 만료승,          
                                sum(decode(매도구분, '기간만료', decode(sign(매도수량*매도단가 - 매수수량*매수단가), -1, 1, 0), 0)) 만료패,          
                                sum(매수단가*매수수량*0.0015+매도단가*매도수량*0.0015) 세금,          
                                sum(매도단가*매도수량 - 매수단가*매수수량) 차액,
                                sum(매수수량*매수단가) 매수금액,          
                                sum(매도수량*매도단가) 매도금액          
                        from     {}          
                        where      매도단가 is not null   
                            and    비교일자 = decode(:비교일자1, '', 비교일자, :비교일자2)          
                            and    주기준 = decode(:주기준1, '', 주기준, :주기준2)          
                            and    확률 = decode(:확률1, '', 확률, :확률2)          
                            and    건수 = decode(:건수1, '', 건수, :건수2)          
                            and    상승기준 = decode(:상승기준1, '', 상승기준, :상승기준2)          
                            and    상승판매 = decode(:상승판매1, '', 상승판매, :상승판매2)          
                            and    하락판매 = decode(:하락판매1, '', 하락판매, :하락판매2)          
                            and    보유일 = decode(:보유일1, '', 보유일, :보유일2)         
                            and    (매도단가/매수단가) < 2
                        group by 비교일자, 주기준,          
                                확률,          
                                건수,          
                                상승기준,          
                                상승판매,          
                                하락판매,          
                                보유일          
            ) 메인          
            order by 매매건수               
            ", e_para["테이블이름"]);                   

            let qs ="
            select json_object(
                   '비교일자' value 비교일자,
                   '주기준' value 주기준,
                   '확률' value 확률,
                   '건수' value 건수,
                   '상승기준' value 상승기준,
                   '상승판매' value 상승판매,
                   '하락판매' value 하락판매,
                   '보유일' value 보유일,
                   '매매건수_' value 매매건수,
                   '최종차액_' value 최종차액,
                   '건당금액_' value 건당금액,
                   '승률' value 승률,
                   '승' value 승,
                   '패' value 패,
                   '만료' value 만료,
                   '만료' value 만료승,
                   '만료패' value 만료패,
                   '세금' value 세금,
                   '차액' value 차액,
                   '매수금액' value 매수금액,
                   '매도금액' value 매도금액
                   )
            from   (";
            let qe ="
                )";

            let query_tot = format!("{} {} {}", qs, query, qe);
            db::select_json(db, &query_tot, q_para)
        }          
        "get_medo_detail_code" =>
        {
            let query = format!("
            select   	메인.단축코드,   
                        GET_종목명(메인.단축코드) 종목명,   
                        매수일자,   
                        매도일자,   
                        to_char(to_date(매도일자, 'YYYYMMDD')-to_date(매수일자, 'YYYYMMDD')) 실보유일,
                        매수단가,   
                        매도단가,   
                        매도구분,   
                        (매도단가/매수단가)*100 배율,   
                        매도단가*매도수량 - 매수단가*매수수량 - (매수단가*매수수량*0.0015+매도단가*매도수량*0.0015) 최종차액,
                        매도단가*매도수량 - 매수단가*매수수량 차액,
                        매수단가*매수수량*0.0015+매도단가*매도수량*0.0015 세금,   
                        매수수량,   
                        매수단가*매수수량 매수금액,
                        매도수량,
                        매도단가*매도수량 매도금액
            from {} 메인
            where 매도단가 is not null   
            and    매도구분 <> '판매못함'
            and    매도일자 is not null
            and    비교일자 = decode(:비교일자1, '', 비교일자, :비교일자2)          
            and    주기준 = decode(:주기준1, '', 주기준, :주기준2)          
            and    확률 = decode(:확률1, '', 확률, :확률2)          
            and    건수 = decode(:건수1, '', 건수, :건수2)          
            and    상승기준 = decode(:상승기준1, '', 상승기준, :상승기준2)          
            and    상승판매 = decode(:상승판매1, '', 상승판매, :상승판매2)          
            and    하락판매 = decode(:하락판매1, '', 하락판매, :하락판매2)          
            and    보유일 = decode(:보유일1, '', 보유일, :보유일2) 
            and    (매도단가/매수단가) < 2
            ", e_para["테이블이름"]);                   

            let qs ="
            select json_object(
                '단축코드' value 단축코드,
                '종목명' value 종목명,
                '매수일자_' value 매수일자,
                '매도일자_' value 매도일자,
                '실보유일' value 실보유일,
                '매수단가' value 매수단가,
                '매도단가' value 매도단가,
                '매도구분' value 매도구분,
                '배율' value 배율,
                '최종차액' value 최종차액,
                '차액' value 차액,
                '세금' value 세금,
                '매수수량' value 매수수량,
                '매도수량' value 매도수량,
                '매도금액' value 매도금액
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