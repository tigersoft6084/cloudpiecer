# Cloudpiecer

Cloudpiecer is a Rust-based module designed to deobfuscate Cloudflare's main challenge script by replacing shuffled function names with their original names. This module uses the `neon` library to interact with JavaScript, allowing it to work as a Node.js module.

## Features
- Reads a JavaScript file and performs deobfuscation on obfuscated variable and function names.
- Parses and identifies key components in the script, such as agents and tangents, to restore the original logic.
- Saves the transformed JavaScript code to a specified file.

## Installation
To install Cloudpiecer, first ensure you have Rust installed on your system. You will also need Node.js and the Neon CLI.

1. **Install Rust** (if not already installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   
2. **Install Neon CLI**:
   ```sh
   npm install -g @neon-rs/cli
   ```
   
3. **Clone the Repository**:
   ```sh
   git clone https://github.com/WhityGhost/cloudpiecer.git
   cd cloudpiecer
   ```
   
4. **Install Dependencies**:
   ```sh
   npm install
   ```

## Usage
Cloudpiecer offers two main functions:

1. **`read_file_to_string`**: Reads a file and returns its contents as a string.
2. **`deobfuscate_scopes`**: Deobfuscates a JavaScript file by identifying and replacing obfuscated names with the original names.

### Example Usage in JavaScript
```javascript
const cloudpiecer = require('./index.node');

// Read a JavaScript file into a string
const fileContents = cloudpiecer.read_file_to_string("input.js");

// Deobfuscate and save the result
const deobfuscatedContents = cloudpiecer.deobfuscate_scopes(fileContents);
console.log("Deobfuscated Content:\n", deobfuscatedContents);
```

### Build Commands
- **Build (Debug)**:
  ```sh
  npm run debug
  ```

- **Build (Release)**:
  ```sh
  npm run build
  ```

- **Cross-Platform Build**:
  ```sh
  npm run cross
  ```

- **Test**:
  ```sh
  npm test
  ```

## Project Structure
- `src/`: Contains the Rust source code, including the core deobfuscation logic.
- `package.json`: Defines the package metadata, dependencies, and build scripts.
- `README.md`: Project documentation.
- `Cargo.toml`: Rust dependency manager file.

## How It Works
Cloudpiecer reads obfuscated JavaScript code, parses key elements (like agents and tangents), applies a cipher to decode them, and reconstructs the original code structure. The core logic is written in Rust, and the `neon` library bridges this with JavaScript.

The deobfuscation process involves:
1. **Identifying Agents**: Uses regex to find and collect shuffled function or variable names.
2. **Decoding Tangents**: Deciphers the obfuscated values and replaces them with readable names.
3. **Saving the Output**: Saves the deobfuscated script as `output.js`.

## Dependencies
- [Neon](https://neon-bindings.com): Provides Rust bindings for Node.js.
- [Regex](https://crates.io/crates/regex): Used for regex pattern matching in the Rust code.

## Contributing
Feel free to submit issues, fork the project, and create pull requests. Contributions are welcome!

## License
This project is licensed under the ISC License.

## Author
Created by [YT-Gh0st](https://github.com/WhityGhost).

## Contact & Support
For issues and support, please visit the [GitHub Issues page](https://github.com/WhityGhost/cloudpiecer/issues).

## Disclaimer
This project is intended for educational purposes. Ensure you have permission before attempting to deobfuscate or analyze any JavaScript code protected by third parties.

---

This `README.md` provides an overview of what the project does, how to install and use it, and additional context on its workings and purpose. Adjust any specific details to suit updates in your code or new functionality.