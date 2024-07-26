# Commands: Session

`ripc session start [method]`:

- Start a new study session.
- `[method]`: defaults to `leitner`
- `[diff_box_date]`: today - last_review

1) get all leitner's cards
2) get all Box 1's cards
3) get other boxes if
  - Get all Box 2 if [diff_box_date] >= 2 && <= 3 days
  - Get all Box 3 if [diff_box_date] >= 7 && <= 13 days
  - Get all Box 4 if [diff_box_date] >= 7 

- create a dir `ripc/`
- This will create/update a temporary file (`ripc/sessions/<date>.toml`) with the current state of today's session
- execute `ripc session view` to print today's session
- show the next card to be studied. Runs `ripc` command

- e.g. `/ripc/sessions/<date>.toml`
```
[leitner.boxes]
1 = [`card/id/which/is/the/dir/path/for/the/cards/ripicard.toml/`, `card/id/2`, `card/id/3`] # array of the card ids in box 1
2 = [...]
(...)
5 = [...]
```

- `ripc session view`: Print a report of the cards scheduled for today’s session, showing the breakdown by boxes.

```
$ ripc session view

Leitner Session Report for 2024-06-26

Box 1 (Daily Review):
- path/to/card1
- path/to/card2
- ...

Box 2 (Every 2 Days - Due Today):
- path/to/card3
- ...

Box 3 (Weekly - Not Due Today):
(No cards)

Box 4 (Biweekly - Not Due Today):
(No cards) 

Box 5 (Monthly - Not Due Today):
(No cards)

To review a card, run:
```



- `ripc session progress`: Show the progress of the current session, including which cards have been reviewed and their status.
  - `--compact`: for simple status progress bar
