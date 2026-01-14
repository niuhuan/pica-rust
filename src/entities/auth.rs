use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub name: String,
    pub birthday: String,
    pub gender: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub question1: String,
    pub question2: String,
    pub question3: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseData {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPasswordResult {
    pub question1: String,
    pub question2: String,
    pub question3: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordResult {
    pub password: String,
}
