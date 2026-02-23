// src/lib.rs
// Mindbell Library - Reusable components for mindful task management
use std::io::Write;
use std::io;
// use chrono::Local;

// ====================================
// Input Handling
// ====================================

/// Get input from the user with a prompt
///
/// This function:
/// 1. Displays the prompt
/// 2. Flushes stdout to ensure prompt appears immediately
/// 3. Reads a line from stdin
/// 4. Returns the trimmed input (whitespace removed from both ends)
///
/// #Arguments
/// * `prompt` - The text to display to the user
///
/// # Returns The user's input as a String, with leading/trailing whitespace removed
///
/// # Panics
/// Panics if reading from stdin fails (rare - usually only if stdin is closed)
pub fn get_input(prompt: &str) -> String {
    // Print the promt without a newline
    print!("{}", prompt);

    // Flush ensures the prompt apperas before we wait for input
    // Without this, the prompt might not show until after user types
    io::stdout().flush().unwrap();

    let mut input = String::new();

    // Read a line from stdin, adding it to our String
    // The newline character is included in the read
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // trim() removes whitespace (including the newline) from both ends
    // to_string() converts the &str slice back to an owned String
    input.trim().to_string()
}

/// Parse a duration string into minutes, with a default fallback
/// 
/// This function handles common user input scenarios:
/// - Empty input (just pressed Enter) -> returns default
/// - Valid number -> returns that number
/// - Invalid input (text, negative, decimal) ->returns default
/// - Input with whitespace -> trims first, then parses
///
/// # Arguments
/// * `input` - The string to parse (from user input)
/// * `default` - The value to return if parsing fails or input is empty
///
/// # Returns
/// The parsed number of minutes, or the default value (DEFAULT_DURATION = 25 minutes)
///
/// # Example
/// ```
/// use mindbell::parse_duration;
/// let DEFAULT_DURATION = 25;
///
/// assert_eq!(parse_duration("30", DEFAULT_DURATION), 30);
/// assert_eq!(parse_duration("", DEFAULT_DURATION), DEFAULT_DURATION);
/// assert_eq!(parse_duration(" abcd ", DEFAULT_DURATION), DEFAULT_DURATION);
/// assert_eq!(parse_duration("  45    ", DEFAULT_DURATION), 45);
/// ```

pub fn parse_duration(input: &str, default: u32) -> u32 {
    // Trim whitespace first - allows "   45 " to work
    let trimmed = input.trim();

    // If empty after trimming, return default
    if trimmed.is_empty() {
        return default;
    }

    // Try to parse as u32
    // parse() return Result<u32, ParseIntError>
    // unwrap_or(default) give us the number if OK, or default if Err
    trimmed.parse::<u32>().unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test configuration constants
    const DEFAULT_DURATION: u32 = 25;

    // ----
    // Duration Parsing Tests
    // ----
    #[test]
    fn test_parse_duriation_with_valid_numbers() {
        assert_eq!(parse_duration("1", DEFAULT_DURATION), 1);
        assert_eq!(parse_duration("25", DEFAULT_DURATION), 25);
        assert_eq!(parse_duration("30", DEFAULT_DURATION), 30);
        assert_eq!(parse_duration("45", DEFAULT_DURATION), 45);
        assert_eq!(parse_duration("90", DEFAULT_DURATION), 90);
        assert_eq!(parse_duration("120", DEFAULT_DURATION), 120);
    }

    #[test]
    fn test_parse_duration_with_empty_string() {
        assert_eq!(parse_duration("", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("", 10),10);
    }

    #[test]
    fn test_parse_duration_with_whitespace_only() {
        assert_eq!(parse_duration(" ", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("  ", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("\t", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("\n", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("  \n\t  ", DEFAULT_DURATION), DEFAULT_DURATION);
    }

    #[test]
    fn test_parse_duration_with_surrounding_whitespace() {
        assert_eq!(parse_duration(" 30", DEFAULT_DURATION), 30);
        assert_eq!(parse_duration("30 ", DEFAULT_DURATION), 30);
        assert_eq!(parse_duration(" 30 ", DEFAULT_DURATION), 30);
        assert_eq!(parse_duration("  45 ", DEFAULT_DURATION), 45);
    }

    #[test]
    fn test_parse_duration_with_invalid_input() {
        // Text
        assert_eq!(parse_duration("abc", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("twenty", DEFAULT_DURATION), DEFAULT_DURATION);

        // Mixed text and numbers
        assert_eq!(parse_duration("30min", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("30 minutues", DEFAULT_DURATION), DEFAULT_DURATION);
        
        // Decimal numbers
        assert_eq!(parse_duration("30.5", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("12.0", DEFAULT_DURATION), DEFAULT_DURATION);
    
        // Negative numbers
        assert_eq!(parse_duration("-10", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("-30", DEFAULT_DURATION), DEFAULT_DURATION);

        // Special characters
        assert_eq!(parse_duration("@#$", DEFAULT_DURATION), DEFAULT_DURATION);
        assert_eq!(parse_duration("30!", DEFAULT_DURATION), DEFAULT_DURATION);
    }

    #[test]
    fn test_parse_duration_zero_is_valid() {
        assert_eq!(parse_duration("0", DEFAULT_DURATION), 0);
    }

    #[test]
    fn test_parse_duration_very_large_number() {
        assert_eq!(parse_duration("1000", DEFAULT_DURATION), 1000);
        assert_eq!(parse_duration("9999", DEFAULT_DURATION), 9999);
    }

    #[test]
    fn test_parse_duration_different_defaults() {
        assert_eq!(parse_duration("", 10), 10);
        assert_eq!(parse_duration("", 15), 15);
        assert_eq!(parse_duration("", 30), 30);
        assert_eq!(parse_duration("abc", 60), 60);
    }

}