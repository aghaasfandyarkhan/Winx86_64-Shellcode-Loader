<img width="1254" height="1254" alt="Image" src="https://github.com/user-attachments/assets/0408222d-1e1c-401c-9999-9b3f96ff72e1" />

# Rust Shellcode Loader

A simple shellcode loader written in Rust for **Red Team Operations**.

The basic idea behind this project is pretty straightforward: the program downloads a `shellcode.bin` file from a remote server and saves it to a specific path defined inside `src/main.rs`.

I built this mainly to experiment with Rust, payload handling, and the kind of file-transfer workflow you might encounter while studying malware and offensive security.

> **Note:** Only use this project on systems you own or have explicit permission to test.

## How It Works

### **Medium Article:** [Click here to open the article](https://aghaasfandyarkhan.medium.com/how-shellcode-loaders-use-a-whole-different-logic-than-other-malwares-red-team-operation-6dbe4a0ae49e?sharedUserId=aghaasfandyarkhan)

The loader follows a simple workflow:

1. Connects to the configured remote server.
2. Downloads `shellcode.bin` in specific path.
3. Executes the shellcode in memory.

The URL and destination path can be changed directly in the source code.

### Disclaimer

This project is intended for **educational purposes, Red Team Operations Knowledge and authorized testing**.
Do not use it to deliver or deploy payloads against systems without permission. You are responsible for making sure your use of this project complies with applicable laws, organizational policies, and the scope of your authorization.
