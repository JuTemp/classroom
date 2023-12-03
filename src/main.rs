use std::vec;

use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;

#[tokio::main]
async fn main() {
    // let s_c = r#"{
    //     "fwzt": "cx",
    //     "xqh_id": "1",
    //     "xnm": "2023",
    //     "xqm": "3",
    //     "lh": "01", // 02 笃学楼  01 厚学楼
    //     "jyfs": "0",
    //     "cdj": "",
    //     "sfb": "",
    //     "zcd": "4096", // log2(4096)+1 = 13th week
    //     "xqj": "6", // Wed Thu Sat [3,4,6].join(","")
    //     "jcd": "3", // 1st~10th [1,2,4,8...].sum()
    //     "_search": "false",
    //     "nd": "1701526629985",
    //     "queryModel.showCount": "50",
    //     "queryModel.currentPage": "1",
    //     "queryModel.sortName": "cdmc",
    //     "queryModel.sortOrder": "asc",
    //     "time": "1",
    // }"#;

    match my_post(
        make_params("01", vec![13], vec![7], vec![5,6,7,8,9,10]),
        "1A3F6F45A1D5924C7D18B63EF00182E7",
    )
    .await
    {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };
}

fn make_params(
    building: &str,
    week: Vec<u32>,
    day: Vec<u32>,
    class: Vec<u32>,
) -> serde_json::Value {
    json!({
        "fwzt": "cx",
        "xqh_id": "1",
        "xnm": "2023",
        "xqm": "3",
        "lh": building,
        "jyfs": "0",
        "cdj": "",
        "sfb": "",
        "zcd": week.iter().map(|w| 2_i32.pow(w-1)).sum::<i32>().to_string(),
        "xqj": day.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(","),
        "jcd": class.iter().map(|w| 2_i32.pow(w-1)).sum::<i32>().to_string(),
        "_search": "false",
        "nd": "1701526629985",
        "queryModel.showCount": "50",
        "queryModel.currentPage": "1",
        "queryModel.sortName": "cdmc",
        "queryModel.sortOrder": "asc",
        "time": "1"
    })
}

#[derive(Deserialize, Debug)]
enum MyPostErr {
    ResponseError(String),
    ParseBodyError(String),
    BodyIncomeError(String),
    NoItemsError(String),
    NoCdmcError(String),
}

async fn my_post(body: serde_json::Value, cookie_jsessionid: &str) -> Result<Vec<String>, MyPostErr> {
    reqwest::Client::new()
        .post("https://jwgl.njtech.edu.cn/cdjy/cdjy_cxKxcdlb.html?doType=query&gnmkdm=N2155")
        .headers({
            let mut cookie = HeaderMap::new();
            cookie.insert(
                header::CONTENT_TYPE,
                "application/x-www-form-urlencoded;charset=UTF-8"
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            cookie.insert(
                header::COOKIE,
                format!("JSESSIONID={}", cookie_jsessionid)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            cookie
        })
        .body({
            body.as_object()
                .ok_or(MyPostErr::BodyIncomeError(String::from("BodyIncomeError")))?
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap()))
                .collect::<Vec<String>>()
                .join("&")
        })
        .send()
        .await
        .map_err(|err| MyPostErr::ResponseError(err.to_string()))?
        // Response
        .json::<serde_json::Value>()
        .await
        .map_err(|err| MyPostErr::ParseBodyError(err.to_string()))?
        // Body
        .get("items")
        .ok_or(MyPostErr::NoItemsError(String::from("NoItemsError")))?
        .as_array()
        .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
        // Items
        .iter()
        .map(|item| -> Result<String, MyPostErr> {
            Ok(item
                .get("cdmc")
                .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
                .as_str()
                .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
                .to_string())
        })
        .collect::<Result<Vec<String>, MyPostErr>>()
}
