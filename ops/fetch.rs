use std::{
    collections::HashMap,
    str::{Bytes, FromStr},
};

use deno_core::{
    anyhow::Result,
    op, serde,
    serde_json::{self, json, Value},
    ZeroCopyBuf,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Client, Method, Request, RequestBuilder, Url,
};
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Deserialize, Serialize, Serializer,
    __private::de::IdentifierDeserializer,
};

#[op]
pub async fn fetch(rpf: RequestProfile) -> Result<ResponseProfile> {
    // 创建请求
    let client = Client::new();
    let req = build_request_for_profile(rpf, client)?;
    println!("【 req 】==> {:?}", req);

    let res = req.send().await?;
    println!("【 res 】==> {:?}", res);

    let status = res.status().as_u16();
    let headers = res.headers().clone();

    let text = res.text().await?;
    let body = json!(&text);
    println!("【 text 】==> {:?}", text);
    let ress = ResponseProfile {
        status,
        headers,
        body: Some(body),
    };

    Ok(ress)
}

#[derive(Debug, Deserialize)]
pub struct RequestProfile {
    url: String,
    #[serde(deserialize_with = "deserialize_to_method", default = "default_method")]
    method: Method,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    headers: HeaderMap,
    // POST、PUT, 经常可能会有二进制，或者长字节流等类型，所以不建议对他序列化
    body: Option<Value>,
    // body: Option<ZeroCopyBuf>,
    // GET
    #[serde(skip_serializing_if = "empty_json_value", default)]
    query: Option<Value>,
}

// #[derive(Debug, Serialize)]
struct ToParity(HashMap<String, Value>);

impl FromStr for ToParity {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // s: "key=value&key=value"
        let mut map = HashMap::new();
        for pair in s.split('&') {
            let mut kv = pair.split('=');
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            map.insert(key.to_string(), Value::String(value.to_string()));
        }

        Ok(ToParity(map))
    }
}

impl Serialize for ToParity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_serializer = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            map_serializer.serialize_entry(key, value)?;
        }
        map_serializer.end()
    }
}

pub fn build_request_for_profile(rpf: RequestProfile, client: Client) -> Result<RequestBuilder> {
    let url = Url::parse(&rpf.url)?;

    let req = client.request(rpf.method, url).headers(rpf.headers);

    let req = if let Some(query) = rpf.query {
        println!("【 query 】==> {:?}", query);
        let query = match query {
            Value::String(s) => {
                let parity: ToParity = s.parse()?;
                req.query(&parity)
            }
            Value::Object(o) => {
                // 支持 Object 键值对
                req.query(&o)
            }
            _ => panic!("query type not support"),
        };

        query
    } else {
        req
    };

    let req = if let Some(body) = rpf.body {
        let body: Body = match body {
            Value::String(s) => {
                println!("【 String 】==> {:?}", s);
                s.into_bytes().into()
            }
            Value::Object(o) => {
                println!("【 Object 】==> {:?}", o);
                let body = serde_json::to_string(&o).expect("error Object");
                body.into()
            }
            Value::Array(a) => {
                println!("【 Array 】==> {:?}", a);
                let body = serde_json::to_string(&a)?;

                body.into()
            }
            _ => vec![].into(),
        };

        req.body(body)
    } else {
        req
    };

    Ok(req)
}

impl RequestProfile {
    // pub fn json_into(params: Value) -> Result<RequestProfile> {
    //     let mut headers = HeaderMap::new();
    //     let mut url: String = Default::default();
    //     let mut method: String = "GET".to_string();
    //     let mut body: String = Default::default();
    //     let mut query = None;

    //     let params = params.as_object().unwrap();
    //     for (k, v) in params {
    //         println!("k value is: {:?}", k.as_str());
    //         match k.as_str() {
    //             "url" => {
    //                 url = v.to_string();
    //                 println!("【 url 】==> {url}");
    //             }
    //             "method" => {
    //                 let v = v.to_string();
    //                 if v.is_empty() {
    //                     method = v;
    //                 }
    //                 println!("【 method 】==> {:?}", method);
    //             }
    //             "headers" => {
    //                 headers = value_to_hasmap(v.clone()).unwrap();
    //                 println!("【 headers 】==> {:?}", headers);
    //             }
    //             "body" => {
    //                 body = v.to_string();
    //                 println!("【 body 】==> {:?}", body);
    //             }
    //             "query" => {
    //                 query = Some(v.clone());
    //             }
    //             other => {
    //                 println!("Incompatible reqeust options: {other:?}");
    //             }
    //         }
    //     }

    //     let rpf = RequestProfile {
    //         url:"https://dummyjson.com/products/1" Url::parse(&"https://dummyjson.com/products/1")
    //             .expect("error url convert to type of Url"),
    //         method,
    //         // .expect("error call foucntion method::from_str"),
    //         headers,
    //         body: None,
    //         query,
    //     };

    //     Ok(rpf)
    // }
}

fn value_to_hasmap(value: Value) -> Option<HeaderMap> {
    if let Some(map) = value.as_object() {
        let mut result = HeaderMap::new();
        for (key, value) in map {
            result.insert(
                HeaderName::from_str(key.clone().as_str()).unwrap(),
                HeaderValue::from_str(&value.clone().to_string()).unwrap(),
            );
        }
        Some(result)
    } else {
        None
    }
}

fn deserialize_to_method<'de, D>(des: D) -> Result<Method, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let method_str: String = Deserialize::deserialize(des)?;
    let method: Method = method_str
        .to_ascii_uppercase()
        .parse()
        .map_err(serde::de::Error::custom)?;
    // let method: Method = Method::from_bytes(method_str.to_ascii_uppercase().as_str().as_bytes())
    //     .expect("error method type for:{method_str} ");

    Ok(method)
}

fn default_method() -> Method {
    Method::GET
}

#[derive(Debug, Serialize)]
pub struct ResponseProfile {
    status: u16,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    headers: HeaderMap,
    body: Option<Value>,
}

// impl From<String> for RequestProfile {
//     fn from(value: String) -> Self {
//         RequestProfile {
//             url: value,
//             ..Default::default()
//         }
//     }
// }

// fn serialize_body<S>(body: &Body, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     // // 将 Body 的内容读取为字节数组
//     // let bytes = body.as_bytes().unwrap_or_default();

//     // // 序列化为 Base64 字符串
//     // let base64_string = base64::encode(bytes);

//     // // 序列化 Base64 字符串
//     // serializer.serialize_str(&base64_string)

//     todo!()
// }

// fn deserialize_body<'de, D>(deserializer: D) -> Result<Body, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     // // 从字符串反序列化 Base64 字符串
//     // let base64_string: String = serde::Deserialize::deserialize(deserializer)?;

//     // // 解码 Base64 字符串为字节数组
//     // let bytes = base64::decode(&base64_string)
//     //     .map_err(|_| serde::de::Error::custom("Failed to decode Base64 string"))?;

//     // // 创建 Body 对象
//     // let body = Body::from(bytes);

//     // Ok(body)
//     todo!()
// }

// 如果返回结果为false, 将不会序列化该字段
fn empty_json_value(v: &Option<Value>) -> bool {
    // 判断v是否为None，如果是则返回true，否则返回v.is_null()
    v.as_ref().map_or(true, |v| v.is_null() || v.is_object())
}
