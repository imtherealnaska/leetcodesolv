pub fn min_changes(s: String) -> i32 {
    s.chars()
        .step_by(2)
        .zip(s.chars().skip(1).step_by(2))
        .filter(|(a, b)| a != b)
        .count() as i32
}

#[test]
fn test_min_changes_to_beautiful() {
    // Test case 1: Already beautiful with "00" and "11" pattern
    let s = "0011".to_string();
    assert_eq!(min_changes(s), 0);

    // Test case 2: Already beautiful with "11" and "00" pattern
    let s = "1100".to_string();
    assert_eq!(min_changes(s), 0);

    // Test case 3: Requires changes to become beautiful
    let s = "0101".to_string();
    assert_eq!(min_changes(s), 2); // Two changes needed to make it "0011" or "1100".

    // Test case 4: Alternating 0's and 1's (full change required)
    let s = "10101010".to_string();
    assert_eq!(min_changes(s), 4); // Minimum changes to make it either "00110011" or "11001100".

    // Test case 5: Only 1's
    let s = "11111111".to_string();
    // it got this wrong gave this as 4
    assert_eq!(min_changes(s), 0); // Minimum changes to make it either "11001100" or "00110011".

    // Test case 6: Only 0's
    let s = "00000000".to_string();
    assert_eq!(min_changes(s), 0); // Minimum changes to make it either "00110011" or "11001100".

    // Test case 7: Mixed with some changes needed
    let s = "11110000".to_string();
    assert_eq!(min_changes(s), 0); // Already in a beautiful pattern of "11110000".

    // Test case 8: Mixed and odd distribution
    let s = "10011100".to_string();
    assert_eq!(min_changes(s), 2); // Two changes needed to make it "11001100".

    // Test case 9: Edge case - smallest possible string (2 characters)
    let s = "10".to_string();
    assert_eq!(min_changes(s), 1); // One change to make it "11" or "00".

    // Test case 10: Edge case - empty string
    let s = "".to_string();
    assert_eq!(min_changes(s), 0); // No changes needed for an empty string.
}
