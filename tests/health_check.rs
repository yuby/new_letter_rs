//! tests/health_check.rs

use news_letter::startup::run;
use reqwest::Client;
use std::net::TcpListener;

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();

  let server = run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check_works() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}


async fn req(address: String, client: Client, body: &str) -> reqwest::Response {
  client
    .post(&address)
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body.to_string())
    .send()
    .await
    .expect("Failed to execute request.")
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  let address = spawn_app();
  let client = reqwest::Client::new();
  let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
  let url = format!("{}/subscription", &address);
  let resp = req(url, client, body)
    .await;

  assert_eq!(200, resp.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_a400_when_data_is_missing() {
  let app_address = spawn_app();
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];
  let url: String = format!("{}/subscription", &app_address);

  for (invalid_body, error_message) in test_cases {
    let response = req(url.clone(), client.clone(), invalid_body).await;

    assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {}.", error_message);
  }
}