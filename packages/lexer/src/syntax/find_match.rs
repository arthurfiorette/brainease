use lazy_regex::Captures;

use crate::token::{all_tokens, Token};

/// Returns a token (and retrieved captures) that matches the given text, if any.
///
pub fn find_match(text: &str) -> Option<(&'static dyn Token, Captures)> {
    for token in all_tokens() {
        let regex = token.regex();
        let captures = regex.captures(text);

        // Regex match
        if captures.is_some() {
            return captures.map(|captures| (token, captures));
        }
    }

    None
}
