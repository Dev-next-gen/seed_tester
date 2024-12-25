# Seed Tester

![Build Status](https://img.shields.io/github/actions/workflow/status/Dev-next-gen/seed_tester/test.yml?branch=main)
![License](https://img.shields.io/github/license/Dev-next-gen/seed_tester)
![Contributors](https://img.shields.io/github/contributors/Dev-next-gen/seed_tester)
![Last Commit](https://img.shields.io/github/last-commit/Dev-next-gen/seed_tester)
![Open Issues](https://img.shields.io/github/issues/Dev-next-gen/seed_tester)

Seed Tester is a powerful analysis tool designed for developers, researchers, and cryptographers who need to evaluate the quality of random number generators (RNG). By running rigorous statistical tests, it ensures high entropy and randomness, which are critical for cryptography, simulations, and gaming applications.

## Features

- Perform six rigorous statistical tests to evaluate RNG quality.
- Export results in JSON format for further analysis.
- User-friendly interface for managing tests and visualizing results.
- Supports unit testing for validating RNG methods.

## Prerequisites

- **Rust** (stable version recommended)
- **Cargo** (included with Rust installation)

---

## Compilation, Testing, and Execution

### Compilation

To compile the project, ensure you’re in the project’s root directory, then run:
```bash
cargo build --release

This command generates an optimized executable in the target/release folder.
Execution

To run the program and perform all RNG tests on a series of random seeds, use the following command:
cargo run --release

The results are displayed in the console and also saved in a results.json file for future reference.
Unit Tests

To run the unit tests and verify the validity of each statistical test, execute:
cargo test

These tests are designed to validate each individual RNG analysis function.

Explanation of RNG Tests

Each test assesses a different characteristic of the random sequence generated. Here’s an overview of the included tests and their significance:
1. Bit Frequency Test

Measures the frequency of "1" bits in the seeds to evaluate their uniformity.

    Thresholds: The frequency of "1"s should be between 45% and 55%.

2. Sequence Length Test

Calculates the maximum length of consecutive "1" sequences in each seed.

    Thresholds: The sequence length should be between 10 and 18.

3. Bit Periodicity Test

Analyzes the similarity between consecutive seeds by measuring the number of similar bits.

    Thresholds: The acceptable periodicity range is between 30% and 70% similarity.

4. Shannon Entropy Test

Measures the amount of information contained in the bit sequence of seeds.

    Thresholds: Entropy should be between 0.99 and 1.01.

5. Bit Correlation Test

Examines the correlation between consecutive seeds, or the likelihood that adjacent bits are similar.

    Thresholds: Correlation should be between 45% and 55%.

6. Poker Test

Divides seeds into groups of four bits and analyzes the distribution of possible combinations.

    Thresholds: Poker test values should fall between 10 and 20.

### Example Output

After running the program, a `results.json` file is generated with the following structure:
```json
{
  "bit_frequency_test": {
    "score": 0.51,
    "status": "passed"
  },
  "sequence_length_test": {
    "score": 14,
    "status": "passed"
  },
  "shannon_entropy_test": {
    "score": 1.00,
    "status": "passed"
  }
}


Contribute

Contributions are welcome! If you’d like to contribute, please fork the repository and submit a pull request with your changes. Make sure to include detailed commit messages.
License

This project is licensed under the MIT License. See the LICENSE file for details.
