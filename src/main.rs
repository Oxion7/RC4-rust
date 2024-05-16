#![allow(non_snake_case)]
// Import necessary crates for standard input and output
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
    
    // Encrypt or decrypt data
    fn process(&mut self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|&b| b ^ self.next_byte()).collect()
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
    let plaintext = plaintext_input.trim().as_bytes();
    
    // Create RC4 instance
    let mut rc4 = RC4::new(key);
    
    // Encrypt the plaintext
    let ciphertext = rc4.process(plaintext);
    println!("Ciphertext (hex): {:?}", ciphertext.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" "));
    
    // Decrypt the ciphertext
    let mut rc4 = RC4::new(key); // Reinitialize RC4 with the same key
    let decrypted = rc4.process(&ciphertext);
    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}

