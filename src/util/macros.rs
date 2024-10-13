#[macro_export]
macro_rules! match_pool {
    ($a: expr, $b: expr, $c: expr) => {
        match $a {
            Ok(exp) => exp,
            Err(_e) => {
                return NetworkResponse::InternalServerError(Json(Response {
                    error_code: Some(500),
                    message: "Service is temporarily unavailable",
                    data: None,
                }))
            }
        }
    };
}
