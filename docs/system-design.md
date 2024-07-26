# System Design

## User Interaction

### Individual Terminal Commands

- Each command represents a user interaction

### Continuous Process (Interactive Session)

- **Session Start**: The user starts a session with a single command, e.g., `leitner-session start`.
- **Interactive Loop**: The program enters an interactive loop where it presents each card, asks for user input, and processes the result.
- **Session End**: The session ends when all cards for the session are reviewed or the user exits the session.

## Other Specs

### Card

- A card will be a dir and it's whole content
- Dir == card, when dir has `ripcard.toml` in root
- Card id: "path/to/ripicard.toml"

- `ripicard.toml` metadata
  ```toml
  [leitner]
  question_file = "question.txt"
  answer_file = "answer.txt"
  box = 1
  ```

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

### Git integration

- 1 commit per card revised
- Commit msg: "Session <date>: <this-card-review-info>"
```
$ git log
commit 3da9f7d7999e6c8f832ee3b488e1eff191b5b9d5
Session 2023-06-09: path/to/card.toml reviewed successfully, moved to box 3
```
