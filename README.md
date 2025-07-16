# Quadratic Voting System in Rust
The idea behind this project was to build a very simple interactive cli-application, to test what I'm currently learning in rust.

## What is Quadratic Voting?

Quadratic voting is a collective decision-making procedure that allows participants to express not just their preferences, but also the intensity of those preferences. Unlike traditional voting where each person gets one vote per issue, quadratic voting allows voters to allocate multiple "credits" to issues they care about most.

**Key Principle**: The cost of votes increases quadratically, but the voting power increases linearly.
- 1 vote costs 1 credit
- 2 votes cost 4 credits  
- 3 votes cost 9 credits
- And so on...

This system ensures that while people can express strong preferences on issues they care deeply about, they cannot simply "buy" elections due to the increasing marginal cost.

## Features

- 🗳️  **Quadratic Voting Logic**: Implements the mathematical foundation of quadratic voting
- 👥  **Voter Registration**: Register multiple participants in a voting session
- 📋  **Proposal Management**: Add and manage voting proposals
- 💰  **Credit System**: Each voter gets allocated credits to spend on proposals
- 📊  **Results Tracking**: View detailed voting results and individual vote breakdowns
- 🖥️  **Interactive CLI**: User-friendly command-line interface for easy interaction
- ✅  **Session Management**: Control when voting sessions start and end

## Installation & Setup

### Prerequisites
- Rust (1.70.0 or later)
- Cargo package manager

### Installation
1. Clone or download this repository
2. Navigate to the project directory:
```bash
cd quadratic-voting-rs
```

3. Build the project:
```bash
cargo build --release
```

## Usage

### Running the Application
```bash
cargo run
```

### Interactive Menu Options

The application provides a simple menu-driven interface:

1. **Register as a Voter** (Option 1)
   - Enter your name to register
   - You'll be assigned a voter ID and receive the default credits (100)

2. **Vote on Proposals** (Option 2)
   - Enter your voter ID
   - Select a proposal ID (0, 1, or 2 for the default proposals)
   - Specify how many credits you want to spend
   - Your vote power will be calculated as the square root of credits spent

3. **View Results** (Option 3)
   - See all proposals with their total vote counts
   - View individual voter contributions to each proposal

4. **Exit** (Option 4)
   - Quit the application

## Default Proposals

The application comes with three pre-configured proposals:
1. **Scroll meetup** - Organise a scroll meetup
2. **Aztec dev workshop** - Organise an aztec zk workshop for developers  
3. **Ethereum Layer 2 Discussion** - Host a discussion about Ethereum Layer 2 scaling solutions

## Development

### Running Tests
```bash
cargo test
```

### Project Structure
- `src/lib.rs` - Core quadratic voting logic and data structures
- `src/main.rs` - Interactive CLI application
- `tests/unit_tests.rs` - Unit tests for core functionality

### Key Components

- **Session**: Manages the overall voting session and coordinates all components
- **Proposal**: Represents voting proposals with titles, descriptions, and vote tallies
- **Voter**: Represents registered voters with allocated credits
- **Vote**: Records individual vote transactions and credit expenditure

## Mathematical Foundation

The quadratic voting formula ensures fair representation:
- **Vote Power** = √(Credits Spent)
- **Total Cost** = (Desired Votes)²

This creates a natural balance where intense preferences can be expressed, but at increasing cost, preventing vote buying while allowing meaningful preference expression.

## TODO

- [ ] Allow users to update/modify votes after casting
- [ ] Add data persistence (save/load sessions)
- [ ] Improve input validation and error messages
- [ ] Add more comprehensive tests (include un-happy paths)
- [ ] Add Gherkin style comments in tests

## Known Limitations

- **No Role Authorisation**
- **No Data Persistence**: All data is lost when the application exits
- **Fixed Proposals**: Proposals are hard-coded and cannot be changed during runtime
- **No Vote Updates**: Voters cannot modify their votes once cast
- **Console Only**: No graphical user interface
- **Basic Input Validation**: Limited error checking for user inputs


