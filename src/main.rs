use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Cdmc {
    cdmc: String,
}

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
    let s_body: serde_json::Value = json!({
        "fwzt": "cx",
        "xqh_id": "1",
        "xnm": "2023",
        "xqm": "3",
        "lh": "01",
        "jyfs": "0",
        "cdj": "",
        "sfb": "",
        "zcd": "8192",
        "xqj": "6",
        "jcd": "3",
        "_search": "false",
        "nd": "1701526629985",
        "queryModel.showCount": "50",
        "queryModel.currentPage": "1",
        "queryModel.sortName": "cdmc",
        "queryModel.sortOrder": "asc",
        "time": "1"
    });

    match my_post(s_body).await {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };
}

#[derive(Deserialize, Debug)]
enum MyPostErr {
    ResponseError(String),
    ParseBodyError(String),
    BodyIncomeError(String),
    NoItemsError(String),
    NoCdmcError(String),
}

async fn my_post(body: serde_json::Value) -> Result<Vec<String>, MyPostErr> {
    let r_body = reqwest::Client::new()
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
                "JSESSIONID=1A3F6F45A1D5924C7D18B63EF00182E7"
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
        .map_err(|err| MyPostErr::ParseBodyError(err.to_string()))?;
    let v = r_body
        // Body
        .get("items")
        .ok_or(MyPostErr::NoItemsError(String::from("NoItemsError")))?
        .as_array()
        .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?;
    let mut r = Vec::new();
    for item in v {
        r.push(
            item
                // .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
                .get("cdmc")
                .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
                .as_str()
                .ok_or(MyPostErr::NoCdmcError(String::from("NoCdmcError")))?
                .to_string()
        );
    }
    Ok(r)
}
