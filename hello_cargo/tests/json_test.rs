// 基于 https://mp.weixin.qq.com/s/z-HvgnmIAcxK60e9VgjGgg

#[cfg(test)]
mod tests {

    #[test]
    fn test_to_string() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct Address {
            street: String,
            city: String,
        }

        // Some data structure.
        let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
        };

        // Serialize it to a JSON string.
        let j = serde_json::to_string(&address).unwrap();

        // Print, write to a file, or send to an HTTP server.
        println!("toString:{}", j);

        // 再解析回来
        let a: Address = serde_json::from_str(&j).unwrap();
        println!("street:{}, city:{}", a.street, a.city);
        println!("j:{}", j);

    }

    #[test]
    fn test_parse_json() {
        // 这是一个 嵌套的JSON
        let data = r#"
         {
             "name": "John Doe",
             "age": 43,
             "phones": [
                 "+44 1234567",
                 "+44 2345678"
             ]
         }"#;

        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
            phones: Vec<String>,
        }

        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        let p: Person = serde_json::from_str(data).unwrap();

        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }

    #[test]
    fn test_untyped_example() {
        use serde_json::Value;

        // 一样的例子
        let data = r#"
                {
                "code": 200,
                "success": true,
                "payload": {
                    "features": [
                        "awesome",
                        "easyAPI",
                        "lowLearningCurve"
                    ]
                }
            }"#;

        // 将数据字符串解析为serde_json::Value。
        let v: Value = serde_json::from_str(data).unwrap();
        let empt = &vec![];
        println!("===v: {:?}", v);
        v["payload"]["123"]
            .as_array()
            .unwrap_or_else(|| empt)
            .iter()
            .for_each(|f| println!("f{}", f));
    }

    #[test]
    fn test_json_macro() {
        use serde_json::json;

        // The type of `john` is `serde_json::Value`
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                2345678
            ]
        });

        println!("first phone number: {}", john["phones"][0]);
        println!("empty value: {}", john["name123"]);

        // Convert to a string of JSON and print it out
        println!("{}", john.to_string());
    }
}
