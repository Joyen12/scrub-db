// Scrub-DB Core Library
// This is the free, open-source "engine" for database anonymization.
// It provides the fundamental anonymization methods but requires manual configuration.

use fake::faker::internet::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::Fake;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Configuration for anonymization rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_true")]
    pub auto_detect: bool,

    #[serde(default)]
    pub custom_rules: HashMap<String, String>,

    #[serde(default = "default_true")]
    pub preserve_relationships: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_detect: false, // Free version doesn't auto-detect
            custom_rules: HashMap::new(),
            preserve_relationships: true,
        }
    }
}

/// Types of anonymization methods available
#[derive(Debug, Clone, PartialEq)]
pub enum AnonymizationType {
    FakeEmail,
    FakeName,
    FakePhone,
    FakeAddress,
    MaskCreditCard,
    MaskSSN,
    Hash,
    Skip,
}

impl AnonymizationType {
    /// Parse anonymization type from string (from config file)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fake_email" | "email" => Some(Self::FakeEmail),
            "fake_name" | "name" => Some(Self::FakeName),
            "fake_phone" | "phone" => Some(Self::FakePhone),
            "fake_address" | "address" => Some(Self::FakeAddress),
            "mask_credit_card" | "credit_card" => Some(Self::MaskCreditCard),
            "mask_ssn" | "ssn" => Some(Self::MaskSSN),
            "hash" => Some(Self::Hash),
            "skip" => Some(Self::Skip),
            _ => None,
        }
    }
}

/// The core anonymization engine
pub struct Anonymizer {
    hash_cache: HashMap<String, String>,
}

impl Anonymizer {
    pub fn new() -> Self {
        Self {
            hash_cache: HashMap::new(),
        }
    }

    /// Anonymize a value based on the anonymization type
    pub fn anonymize(
        &mut self,
        value: &str,
        anon_type: &AnonymizationType,
        preserve_relationships: bool,
    ) -> String {
        match anon_type {
            AnonymizationType::FakeEmail => {
                if preserve_relationships {
                    self.get_or_generate(value, || SafeEmail().fake())
                } else {
                    SafeEmail().fake()
                }
            }

            AnonymizationType::FakeName => {
                if preserve_relationships {
                    self.get_or_generate(value, || Name().fake())
                } else {
                    Name().fake()
                }
            }

            AnonymizationType::FakePhone => {
                if preserve_relationships {
                    self.get_or_generate(value, || PhoneNumber().fake())
                } else {
                    PhoneNumber().fake()
                }
            }

            AnonymizationType::FakeAddress => {
                if preserve_relationships {
                    self.get_or_generate(value, || format!("{} Main St", (100..9999).fake::<i32>()))
                } else {
                    format!("{} Main St", (100..9999).fake::<i32>())
                }
            }

            AnonymizationType::MaskCreditCard => {
                let len = value.len();
                if len > 4 {
                    format!("****-****-****-{}", &value[len - 4..])
                } else {
                    "****".to_string()
                }
            }

            AnonymizationType::MaskSSN => "***-**-****".to_string(),

            AnonymizationType::Hash => {
                let mut hasher = Sha256::new();
                hasher.update(value.as_bytes());
                format!("{:x}", hasher.finalize())
            }

            AnonymizationType::Skip => value.to_string(),
        }
    }

    /// Get cached value or generate new one (for relationship preservation)
    fn get_or_generate<F>(&mut self, original: &str, generator: F) -> String
    where
        F: FnOnce() -> String,
    {
        self.hash_cache
            .entry(original.to_string())
            .or_insert_with(generator)
            .clone()
    }
}

impl Default for Anonymizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymizer_relationship_preservation() {
        let mut anonymizer = Anonymizer::new();

        let email1 = anonymizer.anonymize("john@example.com", &AnonymizationType::FakeEmail, true);
        let email2 = anonymizer.anonymize("john@example.com", &AnonymizationType::FakeEmail, true);

        assert_eq!(email1, email2);
    }

    #[test]
    fn test_anonymizer_mask_credit_card() {
        let mut anonymizer = Anonymizer::new();
        let masked = anonymizer.anonymize(
            "4532-1234-5678-9010",
            &AnonymizationType::MaskCreditCard,
            false,
        );
        assert_eq!(masked, "****-****-****-9010");
    }

    #[test]
    fn test_anonymizer_mask_ssn() {
        let mut anonymizer = Anonymizer::new();
        let masked = anonymizer.anonymize("123-45-6789", &AnonymizationType::MaskSSN, false);
        assert_eq!(masked, "***-**-****");
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(!config.auto_detect); // Free version
        assert!(config.preserve_relationships);
        assert_eq!(config.custom_rules.len(), 0);
    }

    #[test]
    fn test_anonymization_type_from_str() {
        assert_eq!(
            AnonymizationType::from_str("fake_email"),
            Some(AnonymizationType::FakeEmail)
        );
        assert_eq!(
            AnonymizationType::from_str("email"),
            Some(AnonymizationType::FakeEmail)
        );
        assert_eq!(
            AnonymizationType::from_str("mask_ssn"),
            Some(AnonymizationType::MaskSSN)
        );
        assert_eq!(AnonymizationType::from_str("invalid"), None);
    }
}
