# hcs-lib

## Modules (tick marks complete until further notice)
- [ ] `client_database`
- [x] `client_detect_offline`
- [x] `data`
- [ ] `server_database`
- [ ] ~~`client_detect_live`~~
- [x] `config`
- [ ] ~~`errors`~~
- [x] `logger`
- [x] `protocol`
- [x] `testing_utils`

### `client_database`
- [ ] Handle incoming bytes (i.e. write to/read from file + database update)
- [x] Optimize change events
- [x] Add blank files after each program run
- [ ] Add functionality for file writes and creates when receiving from the server

### `server_database`
- [ ] Handle incoming bytes (i.e. write to/read from file + database insert if necessary)
- [x] Create tables
- [ ] ~~Create enums~~
- [x] Create rust objects
- [x] Add `sqlx` row `.into()` rust object traits
- [x] Create insert queries
- [x] Create read queries 

### `client_detect_live` (to be implemented later)
- [ ] Detect live changes
- [ ] Use `client_database` to save those changes

### `errors` (to be implemented later)
- [ ] Create transmission error types
- [ ] Client side error types
- [ ] Server side error types

This is a very brief run down of what has to be done.