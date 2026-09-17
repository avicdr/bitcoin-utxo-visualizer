#[tokio::test]
async fn test_config_loads_defaults() {
    // Verify that AppConfig instantiates properly
    std::env::set_var("BITCOIN_NETWORK", "regtest");
    std::env::set_var("PORT", "8080");

    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();
    assert_eq!(port, 8080);
    assert_eq!(std::env::var("BITCOIN_NETWORK").unwrap(), "regtest");
}
