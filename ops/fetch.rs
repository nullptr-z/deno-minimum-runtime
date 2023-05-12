use deno_core::{anyhow::Result, op, serde, serde_json::Value};
use reqwest::{header::HeaderMap, Body, Client, Method, Url};
use serde::{Deserialize, Serialize};

#[op]
pub async fn fetch(params: Value) -> Result<ResponseProfile> {
    println!("【 params 】==> {:?}", params);
    let rpf = RequestProfile::json_into(params).unwrap();

    // println!("【 req 】==> {:?}", req);
    // 解析参数
    // let url = Url::parse(&rpf.url);
    // // println!("fetch RequestProfile parse {url:?}");

    // // 创建请求
    let client = Client::new();
    let req = client
        .request(rpf.method, rpf.url)
        .headers(rpf.headers)
        .body(rpf.body)
        .query(&rpf.query)
        .build()?;

    let res = client.execute(req).await?;

    Ok(ResponseProfile {
        status: res.status().as_u16(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    url: Url,
    #[serde(with = "http_serde::method", default)]
    method: Method,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    headers: HeaderMap,
    // POST、PUT, 经常可能会有二进制，或者长字节流等类型，所以不建议直接序列化他
    #[serde(
        serialize_with = "serialize_body",
        deserialize_with = "deserialize_body"
    )]
    body: Body,
    // GET
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub query: Option<Value>,
}

impl RequestProfile {
    pub fn json_into(params: Value) -> Result<RequestProfile> {
        let mut headers = HeaderMap::new();
        let mut body: &str = "";

        let params = params.as_object().unwrap();

        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    status: u16,
    // headers: Option<Value>,
    // body: Option<Value>,
}

// impl From<String> for RequestProfile {
//     fn from(value: String) -> Self {
//         RequestProfile {
//             url: value,
//             ..Default::default()
//         }
//     }
// }

fn serialize_body<S>(body: &Body, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // // 将 Body 的内容读取为字节数组
    // let bytes = body.as_bytes().unwrap_or_default();

    // // 序列化为 Base64 字符串
    // let base64_string = base64::encode(bytes);

    // // 序列化 Base64 字符串
    // serializer.serialize_str(&base64_string)

    todo!()
}

fn deserialize_body<'de, D>(deserializer: D) -> Result<Body, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // // 从字符串反序列化 Base64 字符串
    // let base64_string: String = serde::Deserialize::deserialize(deserializer)?;

    // // 解码 Base64 字符串为字节数组
    // let bytes = base64::decode(&base64_string)
    //     .map_err(|_| serde::de::Error::custom("Failed to decode Base64 string"))?;

    // // 创建 Body 对象
    // let body = Body::from(bytes);

    // Ok(body)
    todo!()
}

// 如果返回结果为false, 将不会序列化该字段
fn empty_json_value(v: &Option<Value>) -> bool {
    // 判断v是否为None，如果是则返回true，否则返回v.is_null()
    v.as_ref().map_or(true, |v| v.is_null() || v.is_object())
}
