## Change Log

### v0.2.4

- Reset password
- Update hidden files when renaming or deleting

This version uses directory `data/db` to store database files instead of `db`, which means:

- Docker command should change accordingly, see `README.md`
- Move `db` directory into `data` directory to keep previous data

### v0.2.3

- Allow guest users login
- Hide files from guest users

### v0.2.2

- File search
- Change modal style

### v0.2.1

- Context menu
- File rename
- File delete
- HTTPS via config file

### v0.2

- File upload
- Folder create
- Character encoding other than UTF
- Local IP address on other platforms

### v0.1.2

- Optionally use config file oasis.conf to specify the IP address and port number
- Prevent program crash when no local IP addresses are retrieved

### v0.1.1

- External media player support via the shared link
- File download
- Refactor range request handler
- Check app version against database version for update needs
- Bug fix: token refresh frequency, update modal overflow

### v0.1

- Initial version with basic functionalities.
