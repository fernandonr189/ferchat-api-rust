#[macro_export]
macro_rules! match_pool {
    ($a: expr, $b: expr, $c: expr) => {
        match $a {
            Ok(exp) => exp,
            Err(_e) => {
                return status::Custom(
                    Status::InternalServerError,
                    Json(Response {
                        error_code: Some($b),
                        message: $c,
                        data: None,
                    }),
                )
            }
        }
    };
}
