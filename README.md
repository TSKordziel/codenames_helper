# Codenames Helper

A cross-platform desktop application designed to assist players of the board game Codenames by providing a digital, responsive 5x5 board for marking card selections. This project serves as a showcase of application architecture and GUI development in Rust.

## 🛠️ Technical Implementation

This application was developed to explore modern application architecture and systems-level programming concepts.

## Core Technologies

Language: Rust

GUI Framework: iced (a simple, clean, and responsive GUI library)

State Management: Implements the Elm Architecture pattern for state management (Model, View, Update, Messages), ensuring clear separation between application state and UI logic.

Randomness: Uses the standard rand crate for secure and efficient word shuffling.

## Architectural Highlights

Reactive State Transitions: The application manages state through defined Message types, demonstrating a functional approach to handling user input and triggering reliable UI updates.

Responsive Layout: The display adapts to the window size, including proportional font scaling based on the maximum word length, which is crucial for a consistent user experience.

Memory Safety: Leveraging Rust's ownership system ensures the application is free from common memory-related bugs and guarantees high performance.

## 🚀 Getting Started

This project is built using Cargo, Rust's build system and package manager.

### Prerequisites

Ensure you have the latest stable version of Rust installed:

```
rustup update
```


### Installation and Run

Clone the Repository:

```
git clone [Your Repository URL]
cd codenames-helper
```


### Create the Word List:
As the application uses include_str!, you must create a file named codenames.txt in the root directory. This file should contain a list of words, with one unique word per line.

Note: This repository includes a placeholder file (codenames.txt) with instructions. The application will not run without a list of at least 25 words.

### Run the Application:

```
cargo run
```

## 🤝 License

This project is licensed under the MIT License.