use deno_core::{
    anyhow::Result,
    error::AnyError,
    op, serde,
    serde_json::{self, json, Value},
    ZeroCopyBuf,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Client, Method, RequestBuilder, Url,
};
use serde::{de::DeserializeOwned, ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::{collections::HashMap, ops::Deref, str::FromStr};

#[op]
pub async fn op_fetch(rpf: RequestProfile) -> Result<ResponseProfile> {
    // 创建请求
    let client = Client::new();
    let req = RequestProfile::build_request_for_profile(rpf, client)?;
    println!("【 req 】==> {:?}", req);

    let res = req.send().await?;

    let status = res.status().as_u16();
    let headers = res.headers().clone();

    let text = res.text().await?;
    let body = json!(&text);

    let res = ResponseProfile {
        status,
        headers,
        body: Some(body),
    };

    Ok(res)
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
    #[serde(skip_serializing_if = "empty_json_value", default)]
    body: Option<Value>,
    // #[serde(skip_serializing_if = "empty_json_zero_buf", default)]
    // body: Option<ZeroCopyBuf>,
    #[serde(skip_serializing_if = "empty_json_value", default)]
    query: Option<Value>,
}

impl RequestProfile {
    pub fn build_request_for_profile(rpf: Self, client: Client) -> Result<RequestBuilder> {
        let url = Url::parse(&rpf.url)?;

        let req = client.request(rpf.method, url).headers(rpf.headers);
        println!("【 req 】==> {:?}", req);

        let req = if let Some(query) = rpf.query {
            println!("【 query 】==> {:?}", query);
            let query = match query {
                Value::String(s) => {
                    let parity: ToParity = s.parse()?;
                    println!("【 parity 】==> {:?}", parity);
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
            println!("【 body 】==> {:?}", body);
            let body: Body = match body {
                Value::String(s) => s.into(),
                Value::Object(o) => {
                    let body = serde_json::to_string(&o).expect("error Object");
                    body.into()
                }
                _ => panic!("body type not support"),
            };

            req.body(body)
        } else {
            req
        };

        Ok(req)
    }
}

// Serialize
#[derive(Debug)]
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

// 如果返回结果为false, 将不会序列化该字段
pub fn empty_json_value(v: &Option<Value>) -> bool {
    // 判断v是否为None，如果是则返回true，否则返回v.is_null()
    v.is_some() && v.as_ref().map_or(true, |v| v.is_null() || v.is_object())
}

// 如果返回结果为false, 将不会序列化该字段
pub fn empty_json_zero_buf(v: &Option<ZeroCopyBuf>) -> bool {
    // 判断v是否为None，如果是则返回true，否则返回v.is_null()
    v.is_some() && v.as_ref().map_or(true, |v| v.is_empty())
}

/// 在单元测试时使用 [u8] 作为比 ZeroCopyBuf 更方便, 所以这里把他写出泛型
#[op]
pub fn op_decode_utf8<T>(buf: T) -> Result<String, AnyError>
where
    T: DeserializeOwned + Deref<Target = [u8]>,
{
    let buf = &*buf;
    // from_utf8_lossy: 将字节流转换为字符串, 如果遇到非法的字节, 则用 � 替代, 而不是 panic
    Ok(String::from_utf8_lossy(buf).into())
}
