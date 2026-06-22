use rig_agent::providers::ProviderClient;

#[test]
fn test_provider_client_creation() {
    let client = ProviderClient::new_openai("mock-key");
    assert!(client.is_ok());
}
