# System Design

<!-- toc -->

- [User Interaction](#user-interaction)
  - [Individual Terminal Commands](#individual-terminal-commands)
  - [Continuous Process (Interactive Session)](#continuous-process-interactive-session)
- [Commands: Individual Terminal](#commands-individual-terminal)
- [Dir / Files Structure](#dir--files-structure)
- [Card](#card)
- [Boxes structures](#boxes-structures)
- [Session](#session)
- [Git integration](#git-integration)
- [Other Specs](#other-specs)

<!-- tocstop -->

## User Interaction

### Individual Terminal Commands

- Each command represents a user interaction

### Continuous Process (Interactive Session)

- **Session Start**: The user starts a session with a single command, e.g., `leitner-session start`.
- **Interactive Loop**: The program enters an interactive loop where it presents each card, asks for user input, and processes the result.
- **Session End**: The session ends when all cards for the session are reviewed or the user exits the session.

## Commands: Individual Terminal

Initialize repository with RipCards files and dir structures

```sh
ripc init
```

[New Cards](/docs/commands-new-cards.md)

```sh
ripc new [path]
```

[Study Interaction](/docs/commands-study-interaction.md)

```sh
# Study interaction
ripc # Next card (today's session): Main command
ripc --next
ripc --skip
ripc --show-answer
ripc --correct/-c
ripc --incorrect/-i
```

[Session Subcommands](/docs/commands-session.md)

```sh
# Session-related commands
ripc session start [method]
ripc session view
ripc session progress [--compact]
```

[Reports](/docs/commands-reports.md)

```sh
# Example report commands
ripc report
```

## Dir / Files Structure

```
ripc/
├── config.toml
└── sessions/
    └── [iso-date].toml
```

## Card

- A card will be a dir and it's whole content
- Dir == card, when dir has `ripcard.toml` in root
- Card id: "path/to/ripcard.toml"
- `ripcard.toml` holds the card's metadata

- New card goes to box 1
- Each card (dir) will have files that will represent the "two card faces"
  - question
  - answer
- If answer is correct: move the card to the next box
- If answer is incorrect: move the card to Box 1

## Boxes structures

- Each card metadata (within `ripicard.toml` file) will hold the information about which box it belongs (Leitner System)
- Boxes metadata will be inside the main system config file `config.toml`

## Session

Order of cards in a session (default):

1) By box number: Box 1, 2, ...
2) Inside a box: Random / Shuffle

## Git integration

- 1 commit per card revised
- Commit msg: "Session <date>: <this-card-review-info>"
```
$ git log
commit 3da9f7d7999e6c8f832ee3b488e1eff191b5b9d5
Session 2023-06-09: Correct! Card 'path/to/card1' moved to Box 2.
```

## Other Specs

- Colored output: Colored output: Use green text for correct answers and red for incorrect ones to provide visual feedback. This could be disabled with a --no-color flag.
