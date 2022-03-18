// use std::collections::HashMap;
// #[tokio::main]
// pub async fn main() -> Result<(), reqwest::Error>{
//     let body = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", body);
//     Ok(())
// }

// use std::collections::HashMap;

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1")?
//         .text();
//     println!("{:#?}", resp);
//     Ok(())
// }

pub fn blocking() {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    println!("{}", body);
}
#[tokio::main]
pub async fn asyncro() {
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let resp = reqwest::get(url).await.unwrap();
    let posts: serde_json::Value = resp.json().await.unwrap();
    println!("{}", posts["title"].as_str().unwrap());
    let hello = "world";
    println!("{hello}");
}
