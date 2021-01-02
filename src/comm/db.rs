//아 이거 웃기네.
//속도가 나올라나 모르겠다.

use actix_web::web;
use json::JsonValue;
use r2d2::Pool;
use r2d2_oracle::oracle::sql_type::ToSql;
use r2d2_oracle::OracleConnectionManager;
use std::collections::HashMap;

pub fn select_one(
    db: web::Data<Pool<OracleConnectionManager>>,
    query: &str,
    sql_jo: HashMap<String, String>,
) -> String {
    let conn = db.get().unwrap();
    let mut para: Vec<(&str, &dyn ToSql)> = Vec::new();

    for p_value in sql_jo.iter() {
        // println!("{}, {}", p_value.0, p_value.1);
        let temp_value: &dyn ToSql = p_value.1;
        para.push((p_value.0, temp_value));
    }

    let rows = conn.query_named(query, &para).unwrap();

    let mut return_str = String::new();
    for row_result in rows {
        let row = row_result.unwrap();
        return_str = row.get(0).unwrap();
        break;
    }

    return return_str;
}

pub fn select_json(
    db: web::Data<Pool<OracleConnectionManager>>,
    query: &str,
    sql_jo: HashMap<String, String>,
) -> json::JsonValue {
    let conn = db.get().unwrap();
    let mut para: Vec<(&str, &dyn ToSql)> = Vec::new();

    for p_value in sql_jo.iter() {
        // println!("{}, {}", p_value.0, p_value.1);
        let temp_value: &dyn ToSql = p_value.1;
        para.push((p_value.0, temp_value));
    }

    let rows = conn.query_named(query, &para).unwrap();

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let mut result_json: JsonValue = JsonValue::new_array();

    for row_result in rows {
        let row = row_result.unwrap();
        let temp_str: String = row.get(0).unwrap();
        let temp_json = json::parse(&temp_str);
        result_json.push(temp_json.unwrap()).unwrap();
    }

    let mut result_json2: JsonValue =
        json::parse(r#"{ "check_result":"good", "data":[] }"#).unwrap();
    result_json2["data"] = result_json;

    // println!("{:?}", result_json2["data"][0]);

    return result_json2;
}

pub fn exec(
    db: web::Data<Pool<OracleConnectionManager>>,
    query: &str,
    sql_jo: HashMap<String, String>,
) -> json::JsonValue {
    let conn = db.get().unwrap();
    let mut para: Vec<(&str, &dyn ToSql)> = Vec::new();

    for p_value in sql_jo.iter() {
        // println!("{}, {}", p_value.0, p_value.1);
        let temp_value: &dyn ToSql = p_value.1;
        para.push((p_value.0, temp_value));
    }

    conn.execute_named(query, &para).unwrap();
    conn.commit().unwrap();
    // let exe_result = conn.commit();
    // println!("{:#?}", exe_result);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let result_json2: JsonValue =
        json::parse(r#"{ "check_result":"good", "data":"성공" }"#).unwrap();
    // println!("{:?}", result_json2);

    return result_json2;
}

pub fn get_no_job_id() -> json::JsonValue {
    let result_json: JsonValue =
        json::parse(r#"{ "check_result":"do not search job_id", "data":[{"err":"err"}] }"#)
            .unwrap();
    return result_json;
}
