#[derive(Debug)]
pub struct AppErrors {
    kind: String,
    message: String,
}

impl From<yup_oauth2::Error> for AppErrors {
    fn from(error: yup_oauth2::Error) -> Self {
        AppErrors {
            kind: "yup_oauth2".to_string(),
            message: error.to_string(),
        }
    }
}
