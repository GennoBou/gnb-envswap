use serde::Deserialize;
use std::collections::HashMap;

// Holds the messages for a single language.
pub type Messages = HashMap<String, String>;

// Represents the entire i18n JSON structure.
#[derive(Deserialize)]
struct I18nConfig {
    en: Messages,
    ja: Messages,
}

// A struct to hold the loaded messages for the current locale.
// This makes it easier to pass around and access messages.
pub struct I18nMessages {
    messages: Messages,
}

impl I18nMessages {
    // Get a message by key, with a fallback to the key itself.
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.messages.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
}

/// Loads UI messages from the embedded JSON file based on the system locale.
pub fn load_messages() -> Result<I18nMessages, String> {
    // Embed the messages.json file directly into the binary at compile time.
    const MESSAGES_JSON: &str = include_str!("../i18n/messages.json");

    let config: I18nConfig = serde_json::from_str(MESSAGES_JSON)
        .map_err(|e| format!("Failed to parse i18n messages: {}", e))?;

    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());

    let messages = if locale.starts_with("ja") {
        config.ja
    } else {
        config.en
    };

    Ok(I18nMessages { messages })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_messages_parses_correctly() {
        // This test ensures that the embedded JSON is valid and can be parsed.
        // It doesn't test the locale detection itself, as that's an external dependency.
        let messages_result = load_messages();
        assert!(messages_result.is_ok());
    }

    #[test]
    fn test_i18n_messages_get() {
        let mut messages = Messages::new();
        messages.insert("hello".to_string(), "world".to_string());
        let i18n_messages = I18nMessages { messages };

        // Test getting an existing key
        assert_eq!(i18n_messages.get("hello"), "world");

        // Test getting a non-existent key (should return the key itself)
        assert_eq!(i18n_messages.get("goodbye"), "goodbye");
    }

    #[test]
    fn test_embedded_json_structure() {
        // Directly test the parsing of the embedded JSON string to ensure
        // both `en` and `ja` keys exist and contain the expected messages.
        const MESSAGES_JSON: &str = include_str!("../i18n/messages.json");
        let config: I18nConfig = serde_json::from_str(MESSAGES_JSON).unwrap();

        assert_eq!(config.en.get("select_variable"), Some(&"Select an environment variable".to_string()));
        assert_eq!(config.ja.get("select_variable"), Some(&"環境変数を選択してください".to_string()));
    }
}
