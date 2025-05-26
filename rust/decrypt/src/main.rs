use anyhow::Result;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{KeyInit, generic_array::GenericArray, stream::DecryptorBE32},
};
use clap::Parser;
use secrecy::{ExposeSecret, SecretString};
use std::{
    fs::File,
    io::{Read, Write},
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
    let mut encrypted_file = File::open(encrypted_file_path)?;
    // Create the output file for writing the decrypted content.
    let mut decrypted_file = File::create(decrypted_file_path)?;

    // Expose the secret key once. This key is used to initialize the cipher.
    let key_bytes = encryption_key.expose_secret();

    // Loop to read and decrypt chunks until the end of the encrypted file is reached.
    loop {
        // Buffer to read the 1-byte nonce length indicator.
        let mut nonce_len_buf = [0u8; 1];
        // Attempt to read the nonce length. If `read_exact` returns an error (e.g.,
        // it hits EOF before reading all bytes), it means we've processed the entire
        // file or encountered an incomplete chunk, so we break the loop.
        if encrypted_file.read_exact(&mut nonce_len_buf).is_err() {
            break; // End of file or incomplete last chunk.
        }
        let nonce_len = nonce_len_buf[0] as usize;

        // Validate the nonce length. Your encryption code uses a fixed 7-byte nonce.
        if nonce_len != 7 {
            return Err(anyhow::anyhow!(
                "Invalid nonce length encountered: expected 7, got {}",
                nonce_len
            ));
        }

        // Buffer to read the 7-byte nonce itself.
        let mut nonce_bytes = [0u8; 7];
        // Read the actual nonce bytes from the file.
        encrypted_file.read_exact(&mut nonce_bytes)?;
        // Convert the nonce bytes into a `GenericArray` which is required by the cipher.
        let nonce = GenericArray::from_slice(&nonce_bytes);

        // Buffer to read the 4-byte encrypted data length indicator.
        let mut encrypted_len_buf = [0u8; 4];
        // Read the encrypted data length (stored as big-endian bytes).
        encrypted_file.read_exact(&mut encrypted_len_buf)?;
        // Convert the big-endian bytes back to a `u32`, then cast to `usize`.
        let encrypted_len = u32::from_be_bytes(encrypted_len_buf) as usize;

        // Create a vector to hold the encrypted chunk data.
        let mut encrypted_chunk = vec![0u8; encrypted_len];
        // Read the actual encrypted data into the buffer.
        encrypted_file.read_exact(&mut encrypted_chunk)?;

        // Initialize the `ChaCha20Poly1305` cipher with the encryption key.
        let cipher = ChaCha20Poly1305::new(key_bytes.as_bytes().into());

        // Create a streaming decryptor using the initialized cipher and the current nonce.
        let mut decryptor = DecryptorBE32::from_aead(cipher, nonce);

        // Prepare a mutable buffer for in-place decryption. The encrypted data will be
        // decrypted directly into this buffer.
        let mut decrypted_chunk = encrypted_chunk.to_vec();
        // Perform the decryption. The first argument is "associated data" (AD).
        // Since your encryption uses `&[]` for AD, we also use `&[]` here.
        decryptor
            .decrypt_next_in_place(&[], &mut decrypted_chunk)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        // Write the resulting decrypted plaintext to the output file.
        decrypted_file.write_all(&decrypted_chunk)?;
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
        "Decrypting '{}' to '{}'...",
        args.source.display(),
        args.destination.display()
    );

    // Validate key length for security
    if args.key.len() != 32 {
        return Err(anyhow::anyhow!(
            "Encryption key must be exactly 32 bytes long."
        ));
    }

    // Convert the key string into a SecretString
    let encryption_key = SecretString::from(args.key);

    // Call the decryption function
    match decrypt_file(&args.source, &args.destination, encryption_key) {
        Ok(_) => println!(
            "Decryption complete. Check '{}'",
            args.destination.display()
        ),
        Err(e) => eprintln!("Error during decryption: {}", e),
    }

    Ok(())
}
