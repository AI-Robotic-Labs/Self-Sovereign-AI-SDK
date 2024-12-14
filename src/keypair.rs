use ring::signature::{KeyPair, Ed25519KeyPair, UnparsedPublicKey};
use ring::rand::{SystemRandom, SecureRandom};
#[allow(dead_code)]
fn main() -> Result<(), ring::error::Unspecified> {
    // Random number generator
    let rng = SystemRandom::new();

    // Generate keypair
    let mut seed = [0u8; 32];
    rng.fill(&mut seed)?; // Fill the seed with random bytes
    let keypair = Ed25519KeyPair::from_seed_unchecked(&seed)?;

    // Message to be signed
    let message = b"Secure message for Self-Sovereign AI SDK";

    // Sign the message
    let signature = keypair.sign(message);
    println!("Signature: {:?}", signature.as_ref());

    // Verify the signature
    let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, keypair.public_key().as_ref());
    match public_key.verify(message, signature.as_ref()) {
        Ok(_) => println!("Signature verified successfully!"),
        Err(_) => println!("Failed to verify signature."),
    }

    // Output public and private keys (for demonstration purposes)
    println!("Public Key: {:?}", keypair.public_key().as_ref());
    println!("Private Key Seed: {:?}", seed);

    Ok(())
}
pub fn generate_and_verify_keypair() -> Result<(), ring::error::Unspecified> {
    // Random number generator
    let rng = SystemRandom::new();

    // Generate keypair
    let mut seed = [0u8; 32];
    rng.fill(&mut seed)?; // Fill the seed with random bytes
    let keypair = Ed25519KeyPair::from_seed_unchecked(&seed)?;

    // Message to be signed
    let message = b"Secure message for Self-Sovereign AI SDK";

    // Sign the message
    let signature = keypair.sign(message);
    println!("Signature: {:?}", signature.as_ref());

    // Verify the signature
    let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, keypair.public_key().as_ref());
    match public_key.verify(message, signature.as_ref()) {
        Ok(_) => println!("Signature verified successfully!"),
        Err(_) => println!("Failed to verify signature."),
    }

    // Output public and private keys (for demonstration purposes)
    println!("Public Key: {:?}", keypair.public_key().as_ref());
    println!("Private Key Seed: {:?}", seed);

    Ok(())
}