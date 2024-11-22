Seed Tester

Seed Tester is an analysis tool designed to evaluate the quality of random number generators (RNG). It runs a series of statistical tests on randomly generated seeds to assess various characteristics of their distribution and sequence, helping to verify entropy and ensure randomness.
Compilation, Testing, and Execution
Prerequisites

    Rust (stable version recommended)
    Cargo (included with Rust installation)

Compilation

To compile the project, ensure you’re in the project’s root directory, then run:

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

This test measures the frequency of "1" bits in the seeds to evaluate their uniformity. A frequency close to 50% indicates a good balance between "1" and "0", suggesting a uniform random distribution.

    Thresholds: The frequency of "1"s should be between 45% and 55% to pass this test.

2. Sequence Length Test

This test calculates the maximum length of consecutive "1" sequences in each seed. Excessively long or short sequences can indicate a bias in number generation.

    Thresholds: The sequence length should be between 10 and 18 to be considered acceptable.

3. Bit Periodicity Test

The periodicity test analyzes the similarity between consecutive seeds by measuring the number of similar bits. High periodicity might signal correlation, while low periodicity might indicate a non-uniform distribution.

    Thresholds: The acceptable periodicity range is between 30% and 70% similarity.

4. Shannon Entropy Test

Entropy measures the amount of information contained in the bit sequence of seeds. An entropy close to 1 indicates that each bit has an equal probability of being "0" or "1", which is expected in high-quality RNG.

    Thresholds: Entropy should be between 0.99 and 1.01 to pass this test.

5. Bit Correlation Test

This test examines the correlation between consecutive seeds, or the likelihood that adjacent bits are similar. Lower correlation generally indicates higher RNG quality.

    Thresholds: Correlation should be between 45% and 55%.

6. Poker Test

The poker test divides seeds into groups of four bits and analyzes the distribution of possible combinations. This distribution should be relatively uniform in a high-quality RNG, akin to the distribution of poker hands in a fair game.

    Thresholds: Poker test values should fall between 10 and 20 to be acceptable.

Results

Each test produces a score and a “passed” or “failed” verdict. A final report is generated and saved in results.json, summarizing each test’s score and pass status.

This README.md file provides an overview of your project, usage instructions, and detailed explanations of each statistical test conducted on the RNG.
