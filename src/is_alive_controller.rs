use std::sync::Arc;

use async_trait::async_trait;
use my_http_server::{middlewares::controllers::{actions::GetAction, documentation::HttpActionDescription, ControllersMiddleware}, HttpOkResult, HttpFailResult, HttpContext, WebContentType};

use crate::{IsAliveContext, IsAliveResponse};

pub struct IsAliveController {
    app: Arc<IsAliveContext>,
}

impl IsAliveController {
    pub fn new(app: Arc<IsAliveContext>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl GetAction for IsAliveController {
    async fn handle_request(&self, _ctx: HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let response = IsAliveResponse {
            is_alive: self.app.is_alive,
            app_compilation_date: &self.app.app_compilation_date,
            app_version: &self.app.app_version,
            env_info: &self.app.env_info,
            env_variable_sha1: &self.app.variables,
            framework_version: &self.app.framework_version,
        };

        return HttpOkResult::create_json_response(response).into();
    }

    fn get_controller_description(
        &self,
    ) -> my_http_server::middlewares::controllers::documentation::HttpActionDescription {
        HttpActionDescription {
            name: "api/isalive",
            description: "Get IsAlive status",
            out_content_type: WebContentType::Json,
        }
        .into()
    }

    fn get_in_parameters_description(
        &self,
    ) -> Option<Vec<my_http_server::middlewares::swagger::types::SwaggerInputParameter>> {
        None
    }
}

pub fn build(app: Arc<IsAliveContext>) -> ControllersMiddleware {
    let mut controllers = ControllersMiddleware::new();
    let is_alive_controller = Arc::new(IsAliveController { app });

    controllers.register_get_action("/api/isalive", is_alive_controller.clone());

    controllers
}
