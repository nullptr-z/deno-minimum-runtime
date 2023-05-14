use std::str::FromStr;

use deno_core::{anyhow::Result, op, serde, serde_json::Value, ZeroCopyBuf};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Client, Method, Request, RequestBuilder, Url,
};
use serde::{de::Visitor, Deserialize, Serialize};

#[op]
pub async fn fetch(rep: RequestProfile) -> Result<ResponseProfile> {
    // println!("【 params 】==> {:?}", params);
    // let rpf = RequestProfile::json_into(params).unwrap();

    // println!("【 req 】==> {:?}", req);
    // 解析参数
    // let url = Url::parse(&rpf.url);
    // // println!("fetch RequestProfile parse {url:?}");

    // // 创建请求
    let client = Client::new();
    let req = rep.build_request(client)?;
    println!("【 req 】==> {:?}", req);

    // tokio::spawn(async move {
    let res = req.send().await?; //.expect("error is run spawn task");
    println!("【 res 】==> {:?}", res);
    // });

    Ok(ResponseProfile {
        status: res.status().as_u16(),
    })
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
    // #[serde(
    //     serialize_with = "serialize_body",
    //     deserialize_with = "deserialize_body"
    // )]
    body: Option<ZeroCopyBuf>,
    // GET
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub query: Option<Value>,
}

impl RequestProfile {
    pub fn build_request(&self, client: Client) -> Result<RequestBuilder> {
        println!("【 self.headers 】==> {:?}", self.headers);
        // let method = Method::from_str(self.method.to_ascii_uppercase().as_str())?;
        let url = Url::parse(&self.url)?;
        let req = client.request(self.method.clone(), url)
        .headers(self.headers.clone())
        // .body(self.body.unwrap())
        // .query(&self.query);
        ;

        Ok(req)
    }

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
