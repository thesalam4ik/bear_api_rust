use std::collections::HashMap;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::{Client, Url};
use tokio::task::JoinHandle;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum CaptchaType {
    V1,  // Общая модель для капч, где написан англ. текст
    V2,  // Капчи русским текстом (Например: двести сорок три)
    V3,  // Капчи с русскими буквами (Например: чхЩ, рРп)
    V4,  // Капчи-Пазлы, для того чтобы бот крутил рамки NullCordX (как MineLegacy)
    V5,  // Капчи, где нужно крутить 3x3 рамки чтобы увидеть текст
    V52, // Капчи с русским текстом формата 3x3, где нужно вращение
    V53, // Капчи 2x2, где нужно вращать рамки, чтобы увидеть текст
    V6,  // Модель капч, обученная на капчах с сайтов
    B1,  // Обход капчи из блоков перед игроком (Требует изображение созданное из блоков)
}

#[allow(dead_code)]
impl CaptchaType {
    fn as_prefix(self) -> &'static str {
        match self {
            CaptchaType::V1 => "",
            CaptchaType::V2 => "V2_",
            CaptchaType::V3 => "V3_",
            CaptchaType::V4 => "V4_",
            CaptchaType::V5 => "V5_",
            CaptchaType::V52 => "V52_",
            CaptchaType::V53 => "V53_",
            CaptchaType::V6 => "V6_",
            CaptchaType::B1 => "B1_",
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SolveError {
    Request(reqwest::Error),
    BadResponse(String),
    MissingCaptchaId,
    MissingResult,
}

impl From<reqwest::Error> for SolveError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

#[allow(dead_code)]
#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct BareAPI {
    #[builder(default = r#"Url::parse("http://5.42.211.111/").unwrap()"#)]
    url: Url,

    #[builder(default = "CaptchaType::V1")]
    captcha_type: CaptchaType,

    #[builder(
        default = "Client::builder().timeout(Duration::from_secs(5)).build().expect(\"reqwest client\")"
    )]
    client: Client,

    api_key: String,
}

#[allow(dead_code)]
impl BareAPI {
    pub async fn solve<F>(self, base64: String, cb: F) -> JoinHandle<()>
    where
        F: FnOnce(Result<String, SolveError>) + Send + 'static,
    {
        tokio::spawn(async move {
            let send_captcha_url = self.url.join("in.php").expect("bad base url");
            let result_captcha_url = self.url.join("res.php").expect("bad base url");

            let mut form = HashMap::new();
            form.insert(
                "key",
                format!("{}{}", self.captcha_type.as_prefix(), self.api_key),
            );
            form.insert("method", "base64".to_string());
            form.insert("body", base64.to_string());

            let result = async {
                let send_text = self
                    .client
                    .post(send_captcha_url)
                    .form(&form)
                    .send()
                    .await?
                    .text()
                    .await?;

                let captcha_id = send_text
                    .strip_prefix("OK|")
                    .map(|v| v.trim().to_string())
                    .ok_or_else(|| SolveError::BadResponse(send_text.clone()))?;

                if captcha_id.is_empty() {
                    return Err(SolveError::MissingCaptchaId);
                }

                tokio::time::sleep(std::time::Duration::from_millis(600)).await;

                let mut result_form = HashMap::new();
                result_form.insert(
                    "key",
                    format!("{}{}", self.captcha_type.as_prefix(), self.api_key),
                );
                result_form.insert("action", "get".to_string());
                result_form.insert("id", captcha_id);

                let result_text = self
                    .client
                    .get(result_captcha_url)
                    .query(&result_form)
                    .send()
                    .await?
                    .text()
                    .await?;

                let answer = result_text
                    .strip_prefix("OK|")
                    .map(|v| v.trim().to_string())
                    .ok_or_else(|| SolveError::BadResponse(result_text.clone()))?;

                if answer.is_empty() {
                    return Err(SolveError::MissingResult);
                }

                Ok(answer)
            }
            .await;

            cb(result);
        })
    }
}
