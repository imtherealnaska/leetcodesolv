pub struct KMP {
    pattern: Vec<u8>,
    lps: Vec<usize>,
}

impl KMP {
    /// Creates a new KMP instance with the given pattern
    /// Pre-computes the LPS (Longest Proper Prefix which is also Suffix) array
    pub fn new(pattern: &str) -> Self {
        let pattern = pattern.as_bytes().to_vec();
        let lps = if !pattern.is_empty() {
            Self::compute_lps(&pattern)
        } else {
            Vec::new()
        };
        KMP { pattern, lps }
    }

    fn compute_lps(pattern: &[u8]) -> Vec<usize> {
        let mut lps = vec![0; pattern.len()];
        let mut len = 0;
        let mut i = 1;

        while i < pattern.len() {
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        lps
    }

    /// Finds all occurrences of the pattern in the text
    /// Returns a vector of starting indices where pattern is found
    /// Returns empty vector for empty pattern or empty text
    pub fn find_matches(&self, text: &str) -> Vec<usize> {
        // Handle empty cases
        if self.pattern.is_empty() || text.is_empty() {
            return Vec::new();
        }

        let text = text.as_bytes();
        let mut matches = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < text.len() {
            if self.pattern[j] == text[i] {
                i += 1;
                j += 1;
            }

            if j == self.pattern.len() {
                matches.push(i - j);
                j = if j > 0 { self.lps[j - 1] } else { 0 };
            } else if i < text.len() && self.pattern[j] != text[i] {
                if j != 0 {
                    j = self.lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_single_match() {
        let kmp = KMP::new("ABABCABAB");
        let matches = kmp.find_matches("ABABDABACDABABCABAB");
        assert_eq!(matches, vec![10]);
    }

    #[test]
    fn test_kmp_multiple_matches() {
        let kmp = KMP::new("ABA");
        let matches = kmp.find_matches("ABABABABA");
        assert_eq!(matches, vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_kmp_no_match() {
        let kmp = KMP::new("XYZ");
        let matches = kmp.find_matches("ABABABABA");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_kmp_empty_pattern() {
        let kmp = KMP::new("");
        let matches = kmp.find_matches("ABABABABA");
        assert!(matches.is_empty());
    }
}
