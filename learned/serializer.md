
```rs
// 给 str 实现序列化、反序列化器
struct VisitorImpl;

impl<'de> serde::de::Visitor<'de> for VisitorImpl {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        println!("【 value 】==> {:?}", value);
        Ok(value.to_string())
    }
}
// 大多数常见类型都已经内置实现，自定义 Struct 才实现序列化器(某个字段没他的实现), 而且自定义类型可以通过 derive 宏自动生成序列化器
// #[derive(Serialize, Deserialize)]
let method_str = des.deserialize_str(VisitorImpl)?;

// 不过以下方式更为通用，不需要手动实现序列器，需要指定的返回类型：
let method_str: String = Deserialize::deserialize(des)?;

```
