# System Design

## User Interaction

### Individual Terminal Commands

- Each command represents a user interaction

### Continuous Process (Interactive Session)

- **Session Start**: The user starts a session with a single command, e.g., `leitner-session start`.
- **Interactive Loop**: The program enters an interactive loop where it presents each card, asks for user input, and processes the result.
- **Session End**: The session ends when all cards for the session are reviewed or the user exits the session.

## Commands: Individual Terminal

```sh
# Main command
ripc
```

[New Cards](/docs/commands-new-cards.md)

```sh
# Main command
ripc new
ripc init
```

[Session Subcommands](/docs/commands-session.md)

```sh
# Session-related commands
ripc session view
ripc session view --all
ripc session start
ripc session progress [--compact]
```

[Study Interaction](/docs/commands-study-interaction.md)

```sh
# Study interaction
ripc
ripc --skip
ripc --show-answer
ripc --correct/-c
ripc --incorrect/-i
```

[Reports](/docs/commands-reports.md)

```sh
# Example report commands
ripc report
```

## Other Specs

- Colored output: Colored output: Use green text for correct answers and red for incorrect ones to provide visual feedback. This could be disabled with a --no-color flag.

### Card

- A card will be a dir and it's whole content
- Dir == card, when dir has `ripcard.toml` in root
- Card id: "path/to/ripcard.toml"
- `ripcard.toml` metadata

```toml
[content]
question_file = "question.md"
answer_file = "answer.md"
tags = ["biology", "chapter1"]

[leitner]
box = 2
last_review = { "2024-06-25": "correct" }
review_history = [
  { "2024-06-20": "correct" },
  { "2024-06-21": "incorrect" },
  { "2024-06-24": "correct" },
]
```

- New card goes to box 1
- Each card (dir) will have files that will represent the "two card faces"
  - question
  - answer
- If answer is correct: move the card to the next box
- If answer is incorrect: move the card to Box 1

### Boxes structures

- Each card metadata (within `ripicard.toml` file) will hold the information about which box it bellongs (Leitner System)
- The Boxes metadata will be inside the main system config file `ripcards_config.toml`

```toml
[leitner.boxes]
1 = { review_frequency: 'daily', last_review: '2024-06-25' }
2 = { review_frequency: 'every_2_3_days', last_review: '2024-06-25' }
3 = { review_frequency: 'weekly', last_review: '2024-06-25' }
4 = { review_frequency: 'biweekly', last_review: '2024-06-25' }
5 = { review_frequency: 'monthly', last_review: '2024-06-25' }
```

### Session

Order of cards in a session (default):

1) By box number: Box 1, 2, ...
2) Inside a box: Random / Shuffle

### Git integration

- 1 commit per card revised
- Commit msg: "Session <date>: <this-card-review-info>"
```
$ git log
commit 3da9f7d7999e6c8f832ee3b488e1eff191b5b9d5
Session 2023-06-09: path/to/card.toml reviewed successfully, moved to box 3
```
