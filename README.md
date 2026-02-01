# bear_api_rust

Небольшая async-библиотека для отправки base64-капчи на Bear API и получения ответа.

## Что внутри
- `BearAPI` с builder-подходом (через `derive_builder`).
- Отправка капчи на `in.php` и получение результата из `res.php`.
- Поддержка выбора типа капчи через префикс ключа.

## Быстрый старт
Библиотека асинхронная, поэтому нужен `tokio`.

```rust
use bear_api_rust::{BearAPIBuilder, CaptchaType};

#[tokio::main]
async fn main() {
    let api = BearAPIBuilder::default()
        .api_key("ВАШ_API_KEY")
        .captcha_type(CaptchaType::V1)
        .build()
        .unwrap();

    let _handle = api
        .solve("BASE64_СТРОКА".to_string(), |result| {
            match result {
                Ok(answer) => println!("Ответ: {answer}"),
                Err(err) => eprintln!("Ошибка: {err:?}"),
            }
        })
        .await;
}
```
