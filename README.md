### Setup
1. install sqlx cli https://crates.io/crates/sqlx-cli
2. bring postgres container up `docker compose up`
3. create .env with `DATABASE_URL` i.e: 
postgres://postgres:secret@localhost/actix-jwt
4. run `sqlx database create`
5. run `sqlx migrate run`
6. start with `cargo run`

### Example requests
using https://httpie.io/

fetch all users
```
http :8080/api/users
```
