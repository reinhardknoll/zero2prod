use reqwest::Client;
use std::net::TcpListener;
use zero2prod::startup;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = Client::new();
    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_responds_with_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = Client::new();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_responds_with_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    // Run server in background (using random port)
    let host = "127.0.0.1";
    let listener = TcpListener::bind(format!("{host}:0")).expect("Failed to bind random port");
    // Retrieve the assigned port
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{host}:{port}")
}
