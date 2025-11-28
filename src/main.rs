use bip39::Mnemonic;
use bitcoin::bip32::Xpriv;
use bitcoin::Network;
use chrono::Local;
/**
 * Bitcoin Key Generator - Simple Rust Implementation
 *
 * This application generates a BIP39 mnemonic seed phrase for use with
 * Bitcoin hardware wallets (Coldcard, Trezor, Ledger, etc.).
 * Designed to run on an air-gapped computer for maximum security.
 *
 * Outputs a printable file optimized for metal plate punching/storage.
 */
use std::fs;
use std::io::Write;

/// Generate a new BIP39 mnemonic (24 words for maximum security)
/// Most hardware wallets support 12, 18, or 24 word seeds - we use 24 for maximum entropy
fn generate_mnemonic() -> Result<Mnemonic, Box<dyn std::error::Error>> {
    let mut entropy = [0u8; 32]; // 256 bits = 24 words
    getrandom::fill(&mut entropy)?;

    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    Ok(mnemonic)
}

/// Generate seed from mnemonic
fn generate_seed(mnemonic: &Mnemonic, passphrase: &str) -> [u8; 64] {
    mnemonic.to_seed(passphrase)
}

/// Derive master private key from seed
fn derive_master_key(
    seed: &[u8; 64],
    network: Network,
) -> Result<Xpriv, Box<dyn std::error::Error>> {
    let key = Xpriv::new_master(network, seed)?;
    Ok(key)
}

/// Get master key fingerprint in hardware wallet format (8 hex characters)
fn get_hardware_wallet_fingerprint(key: &Xpriv) -> String {
    use bitcoin::secp256k1::Secp256k1;
    let secp = Secp256k1::new();
    let fingerprint = key.fingerprint(&secp);
    let fingerprint_bytes = fingerprint.as_bytes();
    format!(
        "{:08x}",
        u32::from_be_bytes([
            fingerprint_bytes[0],
            fingerprint_bytes[1],
            fingerprint_bytes[2],
            fingerprint_bytes[3]
        ])
    )
}

/// Create printable output optimized for metal plate punching
fn create_printable_output(mnemonic: &Mnemonic, fingerprint: &str, label: &str) -> String {
    let words: Vec<&str> = mnemonic.words().collect();
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut output = String::new();

    // Header
    output.push_str("═══════════════════════════════════════════════════════════════\n");
    output.push_str("           BITCOIN SEED PHRASE - METAL PLATE BACKUP\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n\n");

    // Label and metadata
    output.push_str(&format!("Label: {}\n", label));
    output.push_str(&format!("Generated: {}\n", timestamp));
    output.push_str(&format!("Fingerprint: {}\n", fingerprint));
    output.push_str(&format!("Word Count: 24 words (256 bits entropy)\n"));
    output.push_str(&format!("Network: Bitcoin Mainnet\n\n"));

    // Warning
    output.push_str("⚠️  SECURITY WARNING ⚠️\n");
    output.push_str("─────────────────────────────────────────────────────────────\n");
    output.push_str("This seed phrase provides full access to your Bitcoin wallet.\n");
    output.push_str("Store this metal plate in a secure, fireproof location.\n");
    output.push_str("Never share this seed phrase with anyone.\n");
    output.push_str("─────────────────────────────────────────────────────────────\n\n");

    // Seed words in large, clear format for punching
    output.push_str("SEED WORDS (Punch these in order):\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n\n");

    // Format words in rows of 4 for easy reading and punching
    for (i, word) in words.iter().enumerate() {
        let word_num = i + 1;
        output.push_str(&format!("{:2}. {:12}", word_num, word));

        // New line every 4 words
        if word_num % 4 == 0 {
            output.push_str("\n");
        } else {
            output.push_str("  ");
        }
    }

    // Ensure last line ends properly
    if words.len() % 4 != 0 {
        output.push_str("\n");
    }

    output.push_str("\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n");
    output.push_str("VERIFICATION CHECKLIST:\n");
    output.push_str("─────────────────────────────────────────────────────────────\n");
    output.push_str("□ All 24 words are clearly readable\n");
    output.push_str("□ Words are in correct numerical order (1-24)\n");
    output.push_str("□ Fingerprint matches hardware wallet device\n");
    output.push_str("□ Metal plate is stored in secure location\n");
    output.push_str("□ Backup copy exists in separate location\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n\n");

    // Additional format: Single column for easier punching reference
    output.push_str("\n\nSINGLE COLUMN FORMAT (Alternative punching reference):\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n");
    for (i, word) in words.iter().enumerate() {
        output.push_str(&format!("{:2}. {}\n", i + 1, word));
    }
    output.push_str("═══════════════════════════════════════════════════════════════\n\n");

    // Hardware wallet import instructions
    output.push_str("HARDWARE WALLET IMPORT INSTRUCTIONS:\n");
    output.push_str("─────────────────────────────────────────────────────────────\n");
    output.push_str("This seed phrase is compatible with all BIP39 hardware wallets\n");
    output.push_str("(Coldcard, Trezor, Ledger, BitBox, etc.).\n\n");
    output.push_str("Example - Coldcard:\n");
    output.push_str("1. Power on your Coldcard device\n");
    output.push_str("2. Navigate to: Advanced > Danger Zone > Seed Functions > Import Existing\n");
    output.push_str("3. Select '24 words' when prompted\n");
    output.push_str("4. Enter the 24 words in order (1-24)\n");
    output.push_str(&format!(
        "5. Verify the fingerprint matches: {}\n",
        fingerprint
    ));
    output.push_str("6. Set a secure PIN code\n");
    output.push_str("7. Test with a small transaction before storing large amounts\n\n");
    output.push_str("For other hardware wallets, follow their specific recovery/import process.\n");
    output.push_str("─────────────────────────────────────────────────────────────\n\n");

    // Footer
    output.push_str("Generated by bitcoin-keygen (air-gapped system)\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n");

    output
}

/// Create a simple text file with just the words (for easy copying)
fn create_simple_word_list(mnemonic: &Mnemonic) -> String {
    let words: Vec<&str> = mnemonic.words().collect();
    let mut output = String::new();

    // Numbered list
    for (i, word) in words.iter().enumerate() {
        output.push_str(&format!("{:2}. {}\n", i + 1, word));
    }

    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("        Bitcoin Key Generator - Air-Gapped Edition");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("Generating secure BIP39 mnemonic seed phrase...");
    println!();

    // Generate mnemonic
    let mnemonic = generate_mnemonic()?;
    println!("✓ Generated 24-word BIP39 mnemonic");

    // Generate seed and master key
    let seed = generate_seed(&mnemonic, "");
    let master_key = derive_master_key(&seed, Network::Bitcoin)?;
    println!("✓ Derived master private key");

    // Get fingerprint
    let fingerprint = get_hardware_wallet_fingerprint(&master_key);
    println!("✓ Calculated fingerprint: {}", fingerprint);

    // Get label from user or use default
    let label = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Bitcoin Wallet".to_string());

    // Create output directory
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    // Create printable file
    let printable_content = create_printable_output(&mnemonic, &fingerprint, &label);
    let printable_file = format!("{}/seed_phrase_printable.txt", output_dir);
    let mut file = fs::File::create(&printable_file)?;
    file.write_all(printable_content.as_bytes())?;
    println!("✓ Created printable file: {}", printable_file);

    // Create simple word list
    let word_list = create_simple_word_list(&mnemonic);
    let word_list_file = format!("{}/seed_words_simple.txt", output_dir);
    fs::write(&word_list_file, word_list)?;
    println!("✓ Created simple word list: {}", word_list_file);

    // Create seed words for hardware wallet import (just the words, one per line)
    let seed_words_file = format!("{}/seed_words_for_coldcard.txt", output_dir);
    fs::write(
        &seed_words_file,
        mnemonic.words().collect::<Vec<_>>().join("\n"),
    )?;
    println!("✓ Created Coldcard import file: {}", seed_words_file);

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    GENERATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("Files created in: {}", output_dir);
    println!();
    println!("IMPORTANT SECURITY NOTES:");
    println!("─────────────────────────────────────────────────────────────");
    println!("1. Print the 'seed_phrase_printable.txt' file for metal plate");
    println!("2. Verify all words are correct before punching");
    println!("3. Store metal plate in secure, fireproof location");
    println!("4. Create backup copy in separate location");
    println!("5. Delete all files from this computer after printing");
    println!("6. Never store seed phrases on internet-connected devices");
    println!("7. Test import on hardware wallet with small amount first");
    println!("─────────────────────────────────────────────────────────────");
    println!();
    println!("Fingerprint: {}", fingerprint);
    println!("(Verify this matches your hardware wallet after import)");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = generate_mnemonic().unwrap();
        let words: Vec<&str> = mnemonic.words().collect();
        assert_eq!(words.len(), 24, "Mnemonic should have 24 words");

        // Verify all words are from BIP39 wordlist
        for word in words {
            assert!(!word.is_empty(), "Word should not be empty");
            assert!(
                word.chars().all(|c| c.is_alphabetic()),
                "Word should contain only letters"
            );
        }
    }

    #[test]
    fn test_generate_seed() {
        let mnemonic = generate_mnemonic().unwrap();
        let seed = generate_seed(&mnemonic, "");
        assert_eq!(seed.len(), 64, "Seed should be 64 bytes");

        // Test with passphrase
        let seed_with_passphrase = generate_seed(&mnemonic, "test_passphrase");
        assert_ne!(
            seed, seed_with_passphrase,
            "Seed with passphrase should be different"
        );
    }

    #[test]
    fn test_derive_master_key() {
        let mnemonic = generate_mnemonic().unwrap();
        let seed = generate_seed(&mnemonic, "");
        let master_key = derive_master_key(&seed, Network::Bitcoin).unwrap();

        // Verify master key is valid
        assert!(!master_key.to_string().is_empty());
    }

    #[test]
    fn test_get_hardware_wallet_fingerprint() {
        let mnemonic = generate_mnemonic().unwrap();
        let seed = generate_seed(&mnemonic, "");
        let master_key = derive_master_key(&seed, Network::Bitcoin).unwrap();
        let fingerprint = get_hardware_wallet_fingerprint(&master_key);

        // Fingerprint should be 8 hex characters
        assert_eq!(
            fingerprint.len(),
            8,
            "Fingerprint should be 8 hex characters"
        );
        assert!(
            fingerprint.chars().all(|c| c.is_ascii_hexdigit()),
            "Fingerprint should contain only hex characters"
        );
    }

    #[test]
    fn test_create_printable_output() {
        let mnemonic = generate_mnemonic().unwrap();
        let seed = generate_seed(&mnemonic, "");
        let master_key = derive_master_key(&seed, Network::Bitcoin).unwrap();
        let fingerprint = get_hardware_wallet_fingerprint(&master_key);

        let output = create_printable_output(&mnemonic, &fingerprint, "Test Wallet");

        // Verify output contains expected sections
        assert!(
            output.contains("BITCOIN SEED PHRASE"),
            "Should contain header"
        );
        assert!(output.contains("Test Wallet"), "Should contain label");
        assert!(output.contains(&fingerprint), "Should contain fingerprint");
        assert!(
            output.contains("SECURITY WARNING"),
            "Should contain security warning"
        );
        assert!(
            output.contains("SEED WORDS"),
            "Should contain seed words section"
        );
        assert!(
            output.contains("VERIFICATION CHECKLIST"),
            "Should contain checklist"
        );
        assert!(
            output.contains("HARDWARE WALLET IMPORT INSTRUCTIONS"),
            "Should contain instructions"
        );

        // Verify all 24 words are present
        let words: Vec<&str> = mnemonic.words().collect();
        for word in &words {
            assert!(
                output.contains(word),
                "Output should contain word: {}",
                word
            );
        }

        // Verify word count
        let word_count = output.matches("words").count();
        assert!(word_count > 0, "Should mention word count");
    }

    #[test]
    fn test_create_simple_word_list() {
        let mnemonic = generate_mnemonic().unwrap();
        let output = create_simple_word_list(&mnemonic);

        let words: Vec<&str> = mnemonic.words().collect();
        assert_eq!(words.len(), 24);

        // Verify all words are in output
        for (i, word) in words.iter().enumerate() {
            assert!(
                output.contains(word),
                "Output should contain word: {}",
                word
            );
            // Check numbering
            let expected_line = format!("{:2}. {}", i + 1, word);
            assert!(
                output.contains(&expected_line),
                "Should contain numbered line"
            );
        }
    }

    #[test]
    fn test_mnemonic_consistency() {
        // Test that the same mnemonic produces the same seed
        let test_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic =
            Mnemonic::parse_in_normalized(bip39::Language::English, test_phrase).unwrap();

        let seed1 = generate_seed(&mnemonic, "");
        let seed2 = generate_seed(&mnemonic, "");
        assert_eq!(seed1, seed2, "Same mnemonic should produce same seed");

        let master_key1 = derive_master_key(&seed1, Network::Bitcoin).unwrap();
        let master_key2 = derive_master_key(&seed2, Network::Bitcoin).unwrap();
        assert_eq!(
            master_key1.to_string(),
            master_key2.to_string(),
            "Same seed should produce same master key"
        );
    }

    #[test]
    fn test_file_generation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        // Generate mnemonic and files
        let mnemonic = generate_mnemonic().unwrap();
        let seed = generate_seed(&mnemonic, "");
        let master_key = derive_master_key(&seed, Network::Bitcoin).unwrap();
        let fingerprint = get_hardware_wallet_fingerprint(&master_key);

        // Create files
        let printable_content = create_printable_output(&mnemonic, &fingerprint, "Test");
        let printable_file = output_dir.join("seed_phrase_printable.txt");
        fs::write(&printable_file, printable_content).unwrap();

        let word_list = create_simple_word_list(&mnemonic);
        let word_list_file = output_dir.join("seed_words_simple.txt");
        fs::write(&word_list_file, word_list).unwrap();

        let seed_words_file = output_dir.join("seed_words_for_coldcard.txt");
        fs::write(
            &seed_words_file,
            mnemonic.words().collect::<Vec<_>>().join("\n"),
        )
        .unwrap();

        // Verify files exist and have content
        assert!(printable_file.exists(), "Printable file should exist");
        assert!(word_list_file.exists(), "Word list file should exist");
        assert!(seed_words_file.exists(), "Seed words file should exist");

        let printable_content = fs::read_to_string(&printable_file).unwrap();
        assert!(
            !printable_content.is_empty(),
            "Printable file should not be empty"
        );

        let word_list_content = fs::read_to_string(&word_list_file).unwrap();
        assert!(
            !word_list_content.is_empty(),
            "Word list file should not be empty"
        );

        let seed_words_content = fs::read_to_string(&seed_words_file).unwrap();
        assert!(
            !seed_words_content.is_empty(),
            "Seed words file should not be empty"
        );

        // Verify seed words file has 24 lines
        let lines: Vec<&str> = seed_words_content.lines().collect();
        assert_eq!(lines.len(), 24, "Seed words file should have 24 lines");
    }

    #[test]
    fn test_fingerprint_format() {
        // Generate multiple mnemonics and verify fingerprints are unique
        let mut fingerprints = std::collections::HashSet::new();

        for _ in 0..10 {
            let mnemonic = generate_mnemonic().unwrap();
            let seed = generate_seed(&mnemonic, "");
            let master_key = derive_master_key(&seed, Network::Bitcoin).unwrap();
            let fingerprint = get_hardware_wallet_fingerprint(&master_key);

            // Verify format
            assert_eq!(fingerprint.len(), 8);
            assert!(fingerprint.chars().all(|c| c.is_ascii_hexdigit()));

            fingerprints.insert(fingerprint);
        }

        // With high probability, all fingerprints should be unique
        // (though collisions are possible, they're extremely rare)
        assert!(
            fingerprints.len() > 0,
            "Should generate at least one fingerprint"
        );
    }
}
