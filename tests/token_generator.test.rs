use koii_token_generator::token_generator::calculate_generated_tokens;

#[test]
fn test_token_generation_calculation() {
    // Verify token generation function behavior
    assert_eq!(calculate_generated_tokens(1000), 100);
    assert_eq!(calculate_generated_tokens(10_000), 1000);
    assert_eq!(calculate_generated_tokens(2_000_000), 1_000_000); // Max cap test
}

#[test]
fn test_zero_input_handling() {
    // Ensure zero input is handled correctly
    assert_eq!(calculate_generated_tokens(0), 0);
}

#[test]
fn test_maximum_token_generation() {
    // Test scenarios that exceed maximum token generation
    let extreme_dump_amount = 20_000_000;
    assert_eq!(
        calculate_generated_tokens(extreme_dump_amount), 
        1_000_000 // Verify strict max cap
    );
}