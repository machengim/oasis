# Setup Process

### First time

1. User runs the program

2. Check whether the database exists, create one if not

3. Listen on `127.0.0.1` and redirect user to `/setup`

4. Ask user about the username and password and storage

6. Update the database according to user input

7. Restart server and listen on `0.0.0.0` and redirect use to the `/`

The dafault db location is `~/.config/oasis/` or `./`.

`/setup` should have authentication script before running.