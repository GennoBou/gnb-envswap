//! Generates PowerShell commands for setting environment variables.

/// Generates a PowerShell command to set an environment variable.
///
/// This function takes a variable name and a value, and returns a string
/// formatted as a PowerShell command. It handles escaping of single quotes
/// in the value, which is crucial for correctness in PowerShell.
///
/// # Arguments
///
/// * `name` - The name of the environment variable (e.g., "API_KEY").
/// * `value` - The value to set for the variable.
///
/// # Examples
///
/// ```
/// let command = generate_powershell_command("MY_VAR", "simple_value");
/// assert_eq!(command, "$Env:MY_VAR = 'simple_value'");
///
/// let command_with_quote = generate_powershell_command("API_KEY", "it's a secret");
/// assert_eq!(command_with_quote, "$Env:API_KEY = 'it''s a secret'");
/// ```
pub fn generate_powershell_command(name: &str, value: &str) -> String {
    // In PowerShell, single quotes within a single-quoted string are escaped by doubling them.
    let escaped_value = value.replace('\'', "''");
    format!("$Env:{} = '{}'", name, escaped_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_powershell_command_simple() {
        let command = generate_powershell_command("MY_VAR", "hello_world");
        assert_eq!(command, "$Env:MY_VAR = 'hello_world'");
    }

    #[test]
    fn test_generate_powershell_command_with_single_quote() {
        let command = generate_powershell_command("API_SECRET", "it's_a_secret");
        assert_eq!(command, "$Env:API_SECRET = 'it''s_a_secret'");
    }

    #[test]
    fn test_generate_powershell_command_with_multiple_quotes() {
        let command = generate_powershell_command("MESSAGE", "Here's Johnny's car!");
        assert_eq!(command, "$Env:MESSAGE = 'Here''s Johnny''s car!'");
    }

    #[test]
    fn test_generate_powershell_command_empty_value() {
        let command = generate_powershell_command("EMPTY_VAR", "");
        assert_eq!(command, "$Env:EMPTY_VAR = ''");
    }

    #[test]
    fn test_generate_powershell_command_no_value_change() {
        let command = generate_powershell_command("NO_SPECIAL_CHARS", "azAZ09-_. /");
        assert_eq!(command, "$Env:NO_SPECIAL_CHARS = 'azAZ09-_. /'");
    }
}
