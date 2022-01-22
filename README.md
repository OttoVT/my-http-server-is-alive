# my-http-server-is-alive


## Example

```
use my_http_server::middlewares::controllers::documentation::HttpActionDescription;
use my_http_server::middlewares::controllers::ControllersMiddleware;
use my_http_server::{
    middlewares::controllers::actions::GetAction, HttpContext, HttpFailResult, HttpOkResult,
};
use my_http_server::{MyHttpServer, WebContentType};
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let app = Arc::new(AppContext {});
    let mut map = HashMap::new();

    map.insert("key-1".to_string(), "val-1".to_string());
    map.insert("key-2".to_string(), "val-2".to_string());
    map.insert("key-3".to_string(), "val-3".to_string());

    let context = IsAliveContext {
        variables: map,
        is_alive: true,
        app_compilation_date: String::from("SomeData"),
        app_version: String::from("1.0.0"),
        env_info: String::from("AleksComp"),
        framework_version: String::from("1.0.0"),
    };
    let is_alive_context = Arc::new(context);
    let mut http_server: MyHttpServer = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8080)));

    let controllers = Arc::new(is_alive_controller::build(is_alive_context.clone()));

    http_server.add_middleware(controllers);
    http_server.start(app.clone());

    let ten_millis = std::time::Duration::from_millis(300_000);
    std::thread::sleep(ten_millis);
}

pub struct AppContext {}

impl rust_extensions::ApplicationStates for AppContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn is_shutting_down(&self) -> bool {
        false
    }
}
```
