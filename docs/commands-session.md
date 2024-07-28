# Commands: Session

## `ripc session start [method]`

- Must be a initialized repo `ripc init`
- Start a new study session.
- `[method]`: defaults to `leitner`

Execution sequence:

1) get all [method]'s cards (session by method)
2) filter by: next_review == today => sessions_cards
3) Organize by boxes
4) Execute print today's session
  - run `ripc session view`
4) Creates basic [Dir / Files Structure](/docs/system-design.md#dir-files-structure)
5) Generate sessions file: `[session_file_iso_date].toml`
6) Commits
7) Show the next card to be studied.
  - Runs `ripc` command

## `ripc session view`

Print a report of the cards scheduled for today’s session, showing the breakdown by boxes. -> `./model-outputs/session-view`

## `ripc session progress`

Show the progress of the current session, including which cards have been reviewed and their status.
  - `--compact`: for simple status progress bar
