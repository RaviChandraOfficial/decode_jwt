use axum::http::StatusCode;
use base64::decode;
use jsonwebtoken::decode_header;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use axum::http::HeaderValue;


fn parser_payload(o: Option<&str>) -> Result<Value, StatusCode> {
  match o {
      None => Err(StatusCode::EXPECTATION_FAILED),
      Some(part) => {
          let decoded = process_part(part)?;
          Ok(decoded)
      }
  }
}


fn process_part(part: &str) -> Result<Value, StatusCode> {
  let decoded = base64::decode_config(part, base64::URL_SAFE).unwrap();
  // let decoded = base64::decode(part).unwrap();
  let decoded = std::str::from_utf8(&decoded).unwrap();
  let decoded = serde_json::from_str::<serde_json::Value>(decoded).unwrap();
  Ok(decoded)
}

// autherization header lo value
fn main(){
    let jwt = "eyJraWQiOiJRdjcxeVwvSlIyRjhMOHlHMjFveVphdXJXR1ZUN3IrZUJHMmRrelhvcWs1QT0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiI4OGUyY2U0Mi02YmQ5LTRmMTItOWMwYy1hM2M0OGI0MTc4YmEiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV95OVhMNFNhdWkiLCJjbGllbnRfaWQiOiIyaXZpamZ1bmN0b2FxY3NvY3M5OGJjaHBjaSIsIm9yaWdpbl9qdGkiOiJmNGU5Y2FhNS1mYTg4LTQ2YzgtYmYxZS1iZDMwOWMwMjBlMzAiLCJldmVudF9pZCI6ImY1NzU3ODc1LTc2ZDQtNDg0ZS04MWYwLTBhYzU5YTIxMDJkNiIsInRva2VuX3VzZSI6ImFjY2VzcyIsInNjb3BlIjoiYXdzLmNvZ25pdG8uc2lnbmluLnVzZXIuYWRtaW4iLCJhdXRoX3RpbWUiOjE3MTIwNjE3MzEsImV4cCI6MTcxMjE0ODEzMSwiaWF0IjoxNzEyMDYxNzMxLCJqdGkiOiI4MjBjODdlZi0wNDExLTRkZjUtYmZkMS01NTdjNzNlMmM0MTQiLCJ1c2VybmFtZSI6InJhdmljaGFuZHJhIn0.Q5j2wJgFTd2zKiPjmHaNthbqO5BQElquCLc0LUmkcdX4y53zpD9TZsTgDNf-igkxB71-lPq1B6FeenYUEYIXNsjJC4XFECJRGBnJtK50-J7XVcKBH6lRqFAlIbmX8LwtHMK6udUMGcWIYJj9-D6e4UAV0Wlwh2JQsrXO4dGFBa5KKb9okpDQ_GrmuScuxtkZvM9gbjNUkeCtQinGAnL2mnfyrW39YShpk-oinI2RxakLykBFvnjSG87fDpTnRjTQotTgEbF13eD92XjqLMoYuBzODhc_FiL7XYk44Nf0kEkdxYgpX4IF6K-gegqb2uupsoS6aYYiD7cFs9xJKjIBDA=";
    let mut splits = jwt.split(".");
    // println!("{:?}",splits);
    let payload = parser_payload(splits.next());
    let payload = parser_payload(splits.nth(0)).unwrap();


 

    let username = payload.get("username").unwrap().as_str().unwrap();

    println!("{:?}",username);

}