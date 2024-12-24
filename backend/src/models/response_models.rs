use serde::Serialize;


#[derive(Serialize)]
pub struct Success <T: Serialize> {
    message: String,
    data: T
} 

impl <T: Serialize> Success <T> {
    pub fn new(data: T) -> String {
        let success = Success{
            message: String::from("success"),
            data
        };

        serde_json::to_string(&success).unwrap()
    }
}


#[derive(Serialize)]
pub struct Error <T: Serialize> {
    message: String,
    error: T
} 

impl <T: Serialize> Error <T> {
    pub fn new(error: T) -> String {
        let err = Error{
            message: String::from("error"),
            error
        };

        serde_json::to_string(&err).unwrap()
    }
}