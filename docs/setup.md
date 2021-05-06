# Setup Process

### First time

1. User runs the program

2. Check whether the database exists, create on if not

3. Redirect one to `/setup`

4. Ask user about the language setting, username and password (mandantory)

5. Ask user about categories, storages, allow-guest (optional)

6. Update the database and scan input storages accordingly

7. Redirect use to the `/index`

The dafault db location is `~/.config/oasis/` or `./`.

`/setup` should have authentication script before running.