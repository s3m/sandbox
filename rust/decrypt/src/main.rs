use anyhow::{Context, Result}; // Import Context trait from anyhow for detailed errors
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{KeyInit, generic_array::GenericArray, stream::DecryptorBE32},
};
use clap::Parser;
use secrecy::{ExposeSecret, SecretString};
use std::{
    fs::File,
    io::{self, Read, Write}, // Import io for specific error types
    path::PathBuf,
};

/// Decrypts a file encrypted with the format:
/// [nonce_length (1 byte)][nonce (7 bytes)][encrypted_data_length (4 bytes)][encrypted_data]
///
/// This function reads the encrypted file chunk by chunk, extracts the nonce and
/// encrypted data length, and then decrypts the corresponding data.
///
/// # Arguments
/// * `encrypted_file_path` - The path to the file to be decrypted.
/// * `decrypted_file_path` - The path where the decrypted content will be saved.
/// * `encryption_key` - The secret encryption key used during encryption.
///
/// # Errors
/// Returns an `anyhow::Error` if file operations fail, if the nonce length is
/// unexpected, or if decryption fails.
pub fn decrypt_file(
    encrypted_file_path: &PathBuf,
    decrypted_file_path: &PathBuf,
    encryption_key: SecretString,
) -> Result<()> {
    // Open the encrypted input file for reading.
    let mut encrypted_file = File::open(encrypted_file_path).with_context(|| {
        format!(
            "Failed to open encrypted input file: {}",
            encrypted_file_path.display()
        )
    })?;

    // Create the output file for writing the decrypted content.
    let mut decrypted_file = File::create(decrypted_file_path).with_context(|| {
        format!(
            "Failed to create decrypted output file: {}",
            decrypted_file_path.display()
        )
    })?;

    // Expose the secret key once. This key is used to initialize the cipher.
    let key_bytes = encryption_key.expose_secret();

    let mut nonce_len_buf = [0u8; 1];
    encrypted_file
        .read_exact(&mut nonce_len_buf)
        .context("Failed to read nonce length")?;
    let nonce_len = nonce_len_buf[0] as usize;
    if nonce_len != 7 {
        return Err(anyhow::anyhow!(
            "Expected nonce length 7, got {}. File may be corrupted or format mismatch.",
            nonce_len
        ));
    }

    let mut nonce_bytes = [0u8; 7];
    encrypted_file
        .read_exact(&mut nonce_bytes)
        .context("Failed to read initial nonce bytes")?;
    let nonce = GenericArray::from_slice(&nonce_bytes);
    eprintln!("[DEBUG] Nonce read: {:?}", nonce_bytes);

    // === CONSTRUCT STREAM DECRYPTOR ONCE ===
    let cipher = ChaCha20Poly1305::new(key_bytes.as_bytes().into());
    let mut decryptor = DecryptorBE32::from_aead(cipher, nonce);

    let mut chunk_idx = 0;
    let mut total_decrypted_bytes = 0u64;

    // === LOOP OVER ENCRYPTED BLOCKS ===
    loop {
        chunk_idx += 1;
        let mut len_buf = [0u8; 4];
        match encrypted_file.read_exact(&mut len_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                eprintln!("[DEBUG] Reached EOF cleanly at chunk {}.", chunk_idx);
                break;
            }
            Err(e) => {
                return Err(e).context(format!(
                    "Chunk {}: Failed to read encrypted length",
                    chunk_idx
                ));
            }
        }

        let chunk_len = u32::from_be_bytes(len_buf) as usize;
        let mut encrypted_chunk = vec![0u8; chunk_len];
        encrypted_file
            .read_exact(&mut encrypted_chunk)
            .with_context(|| format!("Chunk {}: Failed to read encrypted chunk", chunk_idx))?;

        let mut decrypted_chunk = encrypted_chunk.clone();
        decryptor
            .decrypt_next_in_place(&[], &mut decrypted_chunk)
            .map_err(|e| anyhow::anyhow!("Chunk {}: Decryption failed: {:?}", chunk_idx, e))?;

        decrypted_file
            .write_all(&decrypted_chunk)
            .with_context(|| format!("Chunk {}: Failed to write decrypted chunk", chunk_idx))?;

        total_decrypted_bytes += decrypted_chunk.len() as u64;
        eprintln!(
            "[DEBUG] Chunk {}: Decrypted {} bytes. Total: {}",
            chunk_idx,
            decrypted_chunk.len(),
            total_decrypted_bytes
        );
    }

    Ok(())
}

// Define command-line arguments using clap's derive macros
#[derive(Parser, Debug)]
#[command(author, version, about = "A simple file decryptor", long_about = None)]
struct Args {
    /// Path to the encrypted input file
    #[arg(short, long, value_name = "FILE")]
    source: PathBuf,

    /// Path for the decrypted output file
    #[arg(short, long, value_name = "FILE")]
    destination: PathBuf,

    /// The 32-byte encryption key (e.g., "your_secret_encryption_key_32_bytes!")
    #[arg(short, long, value_name = "KEY")]
    key: String,
}

// Main function to demonstrate how to use the `decrypt_file` function.
fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "Attempting to decrypt '{}' to '{}'...",
        args.source.display(),
        args.destination.display()
    );

    // Validate key length for security
    if args.key.len() != 32 {
        return Err(anyhow::anyhow!(
            "Encryption key must be exactly 32 bytes long. Provided key length: {}",
            args.key.len()
        ));
    }

    // Convert the key string into a SecretString
    let encryption_key = SecretString::from(args.key);

    // Call the decryption function
    match decrypt_file(&args.source, &args.destination, encryption_key) {
        Ok(_) => println!(
            "Decryption complete. Check '{}' for the decrypted file.",
            args.destination.display()
        ),
        Err(e) => eprintln!("Fatal error during decryption: {:?}", e), // Use {:?} for anyhow errors to see context chain
    }

    Ok(())
}
