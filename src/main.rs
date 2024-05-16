use std::io::{self, Write};

// RC4 structure to hold state
struct RC4 {
    s: [u8; 256],
    i: usize,
    j: usize,
}

impl RC4 {
    // Create a new RC4 instance with the given key
    fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        let mut j = 0;

        // Initialize state
        for i in 0..256 {
            s[i] = i as u8;
        }

        // Key-Scheduling Algorithm (KSA)
        for i in 0..256 {
            j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
            s.swap(i, j);
        }

        RC4 { s, i: 0, j: 0 }
    }

    // Generate the next byte of the key stream
    fn next_byte(&mut self) -> u8 {
        self.i = (self.i + 1) % 256;
        self.j = (self.j + self.s[self.i] as usize) % 256;
        self.s.swap(self.i, self.j);
        let t = (self.s[self.i] as usize + self.s[self.j] as usize) % 256;
        self.s[t]
    }

    // Encrypt or decrypt data byte-by-byte
    fn process_byte(&mut self, byte: u8) -> u8 {
        byte ^ self.next_byte()
    }
}

fn main() {
    // Read key from user input
    print!("Enter the key: ");
    io::stdout().flush().unwrap();
    let mut key_input = String::new();
    io::stdin().read_line(&mut key_input).expect("Failed to read line");
    let key = key_input.trim().as_bytes();

    // Read plaintext from user input
    print!("Enter the plaintext: ");
    io::stdout().flush().unwrap();
    let mut plaintext_input = String::new();
    io::stdin().read_line(&mut plaintext_input).expect("Failed to read line");
    let plaintext = plaintext_input.trim();

    // Convert plaintext to bytes
    let plaintext_bytes = plaintext.as_bytes();

    // Create RC4 instance for encryption
    let mut rc4_encrypt = RC4::new(key);

    // Encrypt the plaintext and display ciphertext in real-time
    print!("Ciphertext (hex): ");
    io::stdout().flush().unwrap();
    let ciphertext: Vec<u8> = plaintext_bytes.iter().map(|&byte| {
        let encrypted_byte = rc4_encrypt.process_byte(byte);
        print!("{:02x} ", encrypted_byte);
        io::stdout().flush().unwrap();
        encrypted_byte
    }).collect();
    println!();

    // Create RC4 instance for decryption
    let mut rc4_decrypt = RC4::new(key);

    // Decrypt the ciphertext and display plaintext in real-time
    print!("Decrypted: ");
    io::stdout().flush().unwrap();
    let decrypted_bytes: Vec<u8> = ciphertext.iter().map(|&byte| {
        let decrypted_byte = rc4_decrypt.process_byte(byte);
        print!("{}", decrypted_byte as char);
        io::stdout().flush().unwrap();
        decrypted_byte
    }).collect();
    println!();

    // Convert decrypted bytes back to string
    match String::from_utf8(decrypted_bytes) {
        Ok(decrypted_text) => println!("Decrypted text: {}", decrypted_text),
        Err(_) => println!("Decryption failed: Invalid UTF-8 sequence"),
    }
}
