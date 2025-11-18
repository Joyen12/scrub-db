// Scrub-DB Free - Manual Database Anonymization Tool
// Requires manual configuration via scrub-db.yaml

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use scrub_db_core::{Anonymizer, AnonymizationType, Config};
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::path::PathBuf;

/// Database Anonymization Tool - Manual Configuration
#[derive(Parser)]
#[command(name = "scrub-db")]
#[command(about = "Anonymize PII in database dumps using manual configuration", long_about = None)]
#[command(version)]
struct Cli {
    /// Config file path (auto-detects scrub-db.yaml if not specified)
    #[arg(short = 'c', long = "cfg", alias = "config")]
    config: Option<PathBuf>,

    /// Force stdin mode (auto-detected by default)
    #[arg(long = "stdin")]
    use_stdin: bool,

    /// Subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan SQL dump for potential PII (Pro feature teaser)
    Scan,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle scan command (Pro teaser)
    if let Some(Commands::Scan) = cli.command {
        return handle_scan_command();
    }

    // Determine if we're in stdin mode
    let stdin_mode = cli.use_stdin || !io::stdin().is_terminal();

    if !stdin_mode {
        eprintln!("🔍 Scrub-DB Free - Manual Database Anonymization Tool");
        eprintln!("====================================================\n");
        eprintln!("⚠️  This is the FREE version. It requires manual configuration.");
        eprintln!("    Create a scrub-db.yaml file to specify anonymization rules.\n");
        eprintln!("💡 Want automatic PII detection? Upgrade to Scrub-DB Pro!");
        eprintln!("   Visit https://scrub-db.com for pricing.\n");
        return Ok(());
    }

    // Auto-detect config file in current directory
    let config_path = if let Some(path) = cli.config {
        Some(path)
    } else {
        ["scrub-db.yaml", ".scrub-db.yaml", "scrub-db.yml", ".scrub-db.yml"]
            .iter()
            .find(|name| PathBuf::from(name).exists())
            .map(PathBuf::from)
    };

    // Load config
    let config = if let Some(config_path) = &config_path {
        let config_str = std::fs::read_to_string(config_path)
            .context(format!("Failed to read config file: {:?}", config_path))?;
        eprintln!("📄 Using config: {:?}", config_path);
        serde_yaml::from_str(&config_str).context("Failed to parse config file")?
    } else {
        eprintln!("⚠️  No config file found!");
        eprintln!("   Create scrub-db.yaml with anonymization rules.");
        eprintln!("   Example:");
        eprintln!("   ```yaml");
        eprintln!("   preserve_relationships: true");
        eprintln!("   custom_rules:");
        eprintln!("     users.email: fake_email");
        eprintln!("     users.phone: fake_phone");
        eprintln!("   ```\n");
        eprintln!("💡 Or use `scrub-db scan` to see what PII was detected (Pro feature preview)\n");
        Config::default()
    };

    eprintln!("📥 Reading SQL dump from stdin...");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    // Initialize anonymizer
    let mut anonymizer = Anonymizer::new();

    // Build regex patterns from custom rules
    let mut rules: Vec<(Regex, AnonymizationType)> = Vec::new();
    for (pattern, method_str) in &config.custom_rules {
        if let Some(anon_type) = AnonymizationType::from_str(method_str) {
            // Convert table.column pattern to regex
            let regex_pattern = format!(r"\b{}\b", regex::escape(pattern));
            if let Ok(regex) = Regex::new(&regex_pattern) {
                rules.push((regex, anon_type));
            }
        }
    }

    if rules.is_empty() {
        eprintln!("⚠️  No anonymization rules defined!");
        eprintln!("   Data will pass through unchanged.");
        eprintln!("   Add custom_rules to your scrub-db.yaml file.\n");
    } else {
        eprintln!("✅ Loaded {} anonymization rules", rules.len());
    }

    // Process SQL dump line by line
    let mut line_count = 0;
    for line in reader.lines() {
        let line = line?;
        let mut anonymized_line = line.clone();

        // Simple pattern matching for common PII in INSERT statements
        // This is basic - real pattern matching happens via config rules

        // Detect emails in the line
        let email_regex = Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap();
        for cap in email_regex.find_iter(&line) {
            let original = cap.as_str();
            // Check if this matches any of our rules
            let anon_type = rules
                .iter()
                .find(|(pattern, _)| pattern.is_match(&line))
                .map(|(_, t)| t)
                .unwrap_or(&AnonymizationType::Skip);

            if matches!(anon_type, AnonymizationType::FakeEmail) {
                let fake = anonymizer.anonymize(original, anon_type, config.preserve_relationships);
                anonymized_line = anonymized_line.replace(original, &fake);
            }
        }

        // Detect phone numbers
        let phone_regex = Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap();
        for cap in phone_regex.find_iter(&line) {
            let original = cap.as_str();
            let anon_type = rules
                .iter()
                .find(|(pattern, _)| pattern.is_match(&line))
                .map(|(_, t)| t)
                .unwrap_or(&AnonymizationType::Skip);

            if matches!(anon_type, AnonymizationType::FakePhone) {
                let fake = anonymizer.anonymize(original, anon_type, config.preserve_relationships);
                anonymized_line = anonymized_line.replace(original, &fake);
            }
        }

        // Write line to stdout
        writeln!(stdout, "{}", anonymized_line)?;
        line_count += 1;
    }

    eprintln!("✅ Processed {} lines!", line_count);

    if rules.is_empty() {
        eprintln!("\n💡 Tip: Want automatic PII detection?");
        eprintln!("   Try: scrub-db scan  (shows what Pro version would detect)");
    }

    Ok(())
}

fn handle_scan_command() -> Result<()> {
    eprintln!("🔍 Scrub-DB Scan - PII Detection Preview");
    eprintln!("=========================================\n");

    eprintln!("📥 Reading SQL dump from stdin...\n");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut potential_emails = 0;
    let mut potential_phones = 0;
    let mut potential_cc = 0;
    let mut line_count = 0;

    // Scan for potential PII patterns
    let email_regex = Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap();
    let phone_regex = Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap();
    let cc_regex = Regex::new(r"\b\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}\b").unwrap();

    for line in reader.lines() {
        let line = line?;
        line_count += 1;

        if email_regex.is_match(&line) {
            potential_emails += 1;
        }
        if phone_regex.is_match(&line) {
            potential_phones += 1;
        }
        if cc_regex.is_match(&line) {
            potential_cc += 1;
        }
    }

    eprintln!("✨ Scan Results:");
    eprintln!("   📧 {} lines with potential email addresses", potential_emails);
    eprintln!("   📱 {} lines with potential phone numbers", potential_phones);
    eprintln!("   💳 {} lines with potential credit card numbers", potential_cc);
    eprintln!("   📄 {} total lines scanned\n", line_count);

    if potential_emails + potential_phones + potential_cc > 0 {
        eprintln!("🚀 Upgrade to Scrub-DB Pro for:");
        eprintln!("   ✅ Automatic PII detection (no config needed)");
        eprintln!("   ✅ Smart column name analysis");
        eprintln!("   ✅ Live database connections");
        eprintln!("   ✅ Database-to-database anonymization");
        eprintln!("   ✅ Compliance reporting\n");
        eprintln!("   Visit https://scrub-db.com for pricing and features.\n");
    } else {
        eprintln!("✅ No obvious PII patterns detected in this dump.\n");
    }

    eprintln!("💡 Free version: Create scrub-db.yaml with manual rules");
    eprintln!("   Example:");
    eprintln!("   ```yaml");
    eprintln!("   custom_rules:");
    eprintln!("     users.email: fake_email");
    eprintln!("     users.phone: fake_phone");
    eprintln!("     orders.credit_card_number: mask_credit_card");
    eprintln!("   ```");

    Ok(())
}
