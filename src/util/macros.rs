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

#[macro_export]
macro_rules! match_response {
    ($match: expr, $dat: ident, $mOk: expr, $dOk: expr,$mErr: expr, $ErrStatus: expr) => {
        match $match {
            Ok($dat) => status::Custom(
                Status::Ok,
                Json(Response {
                    error_code: None,
                    message: $mOk,
                    data: $dOk,
                }),
            ),
            Err(_e) => status::Custom(
                $ErrStatus,
                Json(Response {
                    error_code: None,
                    message: $mErr,
                    data: None,
                }),
            ),
        }
    };
}
