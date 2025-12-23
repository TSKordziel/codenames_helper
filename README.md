# Codenames Helper

A cross-platform desktop application designed to assist players of the board game **Codenames** by providing a digital, responsive 5x5 board for marking card selections. This project serves as a focused showcase of application architecture, GUI development, and systems-level programming in Rust.

## 🎯 Project Purpose

This project was built with two explicit goals:

1. **Learn how to design a GUI layer on top of backend business logic**  
   The application emphasizes a clean separation between core state/logic and presentation, mirroring how real-world desktop or frontend systems interface with non-UI logic.

2. **Solidify practical Rust skills through a complete, user-facing application**  
   Rather than isolated exercises, this project applies Rust concepts—ownership, enums, pattern matching, and message-driven state transitions—in a cohesive, end-to-end program.

While the game itself is simple, the architectural decisions reflect patterns that scale to more complex applications.

## 🛠️ Technical Implementation

This application was developed to explore modern application architecture and systems programming concepts.

## Core Technologies

- **Language:** Rust  
- **GUI Framework:** `iced` — a clean, cross-platform, declarative GUI library  
- **State Management:** Elm Architecture (Model, View, Update, Messages), enforcing a clear boundary between state, logic, and rendering  
- **Randomness:** `rand` crate for efficient and reliable word shuffling  

## Architectural Highlights

- **Message-Driven State Updates**  
  All user interactions flow through explicit `Message` types, ensuring predictable state transitions and making the application easy to reason about and extend.

- **Backend-First Design**  
  Game state and logic are modeled independently of the UI, allowing the GUI to act as a thin, reactive layer over well-defined data structures.

- **Responsive Layout**  
  The interface adapts to window size, including proportional font scaling based on maximum word length to maintain readability across screen sizes.

- **Memory Safety by Construction**  
  Rust’s ownership and type system eliminate entire classes of runtime errors while maintaining performance suitable for real-time UI updates

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
git clone https://github.com/TSKordziel/codenames_helper
cd codenames_helper
```


### Create the Word List:
As the application uses include_str!, you must create a file named codenames.txt in the '/src' directory. This file should contain a list of words, with one unique word per line.

Note: This repository includes a placeholder file (codenames.txt) with instructions in the root directory. The application will not run without a list of at least 25 words.

### Run the Application:

```
cargo run
```

## 🤝 License

This project is licensed under the MIT License.
