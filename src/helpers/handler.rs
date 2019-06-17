use actix_web::HttpResponse;

pub fn handle(result: Result<json::JsonValue, Vec<String>>) -> HttpResponse {
    match result {
        Ok(data) => {
            let res = json::object! { "success" => true, "data" => data }.dump();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        },
        Err(errors) => {
            let res = json::object! { "success" => false, "errors" => errors }.dump();
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(res)
        },
    }
}
