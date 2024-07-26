# Commands: Study Interaction

## `ripc`

- if not session file -> `ripc session start`
- get next card `ripc --skip`
  1. go from box 1 to 5 `session_file.leitner.state.box`
  2. in box: get range of array position `session_file.leitner.state.idx`
  3. index: random number between cards array range
- Prints info about the next card that has to be studied
- Prints out the `question_file` (read at `ripcard.toml`)

## `ripc --show-answer`

- Shows the current cards answer file

## `ripc --show-all`

- Shows the current cards data, question and answer

## `ripc --correct/-c`

- `ripcard.toml`:
  - box += 1
  - review_history.push(last_review)
  - last_review = { [next_review]: "correct" }
  - next_review += `config.leitner.boxes[box][random_arr_pos]`
  - runs `ripc --show-all`
  - runs `ripc session progress --compact`
  - update session state: todo -> done

## `ripc --incorrect/-i`

- `ripcard.toml`:
  - box = 1
  - review_history.push(last_review)
  - last_review = { [next_review]: "incorrect" }
  - next_review += `config.leitner.boxes[box][random_arr_pos]`
  - runs `ripc --show-all`
  - runs `ripc session progress --compact`
  - update session state: todo -> done

## `ripc --next`

- get another random number for the index

## `ripc --skip`

- if `leitner.state` is empty
  - `leitner.state.box` = 1
  - `leitner.state.idx` = random(leitner.boxes.todo[leitner.state.box].range())
- if `leitner.state` has data
  - mark current card as done
  - go to the next card
    - `leitner.state.box` += 1
    - `leitner.state.idx` = random(leitner.boxes.todo[leitner.state.box].range())


