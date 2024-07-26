# Commands: Study Interaction

`ripc`

- if not session file -> `ripc session start`
- Prints info about the next card that has to be studied
- Prints out the `question_file` (read at `ripcard.toml`)

`ripc --show-answer`

- Shows the cards answer to be checked

`ripc -c / -i`

- Mark that card as answered correctly (`-y`) or incorrectly (`-n`)
- Print out the question and the answer
- Print `ripc session progress --compact`
- Update card state
- Update session state
- Print the new card status

```
$ ripc
Question: What is the capital of France?
(Card ID: path/to/card1)

$ ripc --show-answer
Answer: Paris

$ ripc --correct/-c
Correct! Card 'path/to/card1' moved to Box 2.

Question: What is the capital of France?
Answer: Paris

$ ripc --incorrect/-i
Incorrect. Card 'path/to/card2' moved back to Box 1.

Question: What is the largest planet in our solar system?
Answer: Jupiter
```
