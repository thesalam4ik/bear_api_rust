# bare_api_rust

Небольшая async-библиотека для отправки base64-капчи на Bare API и получения ответа.

## Что внутри
- `BareAPI` с builder-подходом (через `derive_builder`).
- Отправка капчи на `in.php` и получение результата из `res.php`.
- Поддержка выбора типа капчи через префикс ключа.

## Быстрый старт
Библиотека асинхронная, поэтому нужен `tokio`.

```rust
use bare_api_rust::{BareAPIBuilder, CaptchaType};

#[tokio::main]
async fn main() {
    let api = BareAPIBuilder::default()
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
