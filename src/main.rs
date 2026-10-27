use regex::Regex;

pub struct SecretScrubber {
    patterns: Vec<Regex>,
}

impl SecretScrubber {
    pub fn new() -> Self {
        let raw = [
            r"ghp_[a-zA-Z0-9]{36}",                     // GitHub Personal Access Token
            r"gho_[a-zA-Z0-9]{36}",                     // GitHub OAuth
            r"AKIA[0-9A-Z]{16}",                        // AWS Access Key ID
            r"AIza[0-9A-Za-z\\-_]{35}",                 // Google API Key
            r"-----BEGIN [A-Z ]+PRIVATE KEY-----",      // RSA/OpenSSH Key
            r"sk_live_[0-9a-zA-Z]{24}",                 // Stripe Live Secret Key
        ];
        let patterns = raw.iter().map(|p| Regex::new(p).unwrap()).collect();
        Self { patterns }
    }

    pub fn sanitize(&self, text: &str) -> (String, usize) {
        let mut result = text.to_string();
        let mut count = 0;
        for pattern in &self.patterns {
            let replaced = pattern.replace_all(&result, "[REDACTED_SECRET]");
            if replaced != result {
                count += pattern.find_iter(&result).count();
                result = replaced.into_owned();
            }
        }
        (result, count)
    }
}

fn main() {
    println!("[Clipboard-Secret-Scrubber] Active token sanitizer running.");
    let scrubber = SecretScrubber::new();
    let sample = "Config: token=ghp_111122223333444455556666777788889999 and aws=AKIAIOSFODNN7EXAMPLE";
    let (cleaned, n) = scrubber.sanitize(sample);
    println!("Found {} secret(s).\nResult: {}", n, cleaned);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_token_scrubbing() {
        let s = SecretScrubber::new();
        let token = "ghp_111122223333444455556666777788889999";
        let (res, cnt) = s.sanitize(token);
        assert_eq!(cnt, 1);
        assert_eq!(res, "[REDACTED_SECRET]");
    }

    #[test]
    fn test_clean_text_remains_intact() {
        let s = SecretScrubber::new();
        let text = "Hello world! Nothing to redact.";
        let (res, cnt) = s.sanitize(text);
        assert_eq!(cnt, 0);
        assert_eq!(res, text);
    }
}
