#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
fn welcome(name: String) -> String {
    format!("Welcome, {}!", name)
}
```