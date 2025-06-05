# Blog API

A simple RESTful API for a blog, built with [Rocket](https://rocket.rs/) and [Diesel](https://diesel.rs/) in Rust.

## Features

- User creation
- Post creation (with tags)
- List posts (with pagination, search, and user info)
- PostgreSQL database

## Endpoints

| Method | Path      | Description              |
|--------|-----------|--------------------------|
| POST   | /users    | Create a new user        |
| POST   | /posts    | Create a new post        |
| GET    | /posts    | List posts (paginated)   |

## Example Requests

### Create User

```sh
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","first_name":"Test","last_name":"User"}'
```

### Create Post

```sh
curl -X POST http://localhost:8000/posts \
  -H "Content-Type: application/json" \
  -d '{"post":{"title":"Hello","body":"World","created_by":1},"tags":["rust","api"]}'
```

### List Posts

```sh
curl "http://localhost:8000/posts?page=1&per_page=10&search=hello"
```

## Database Setup

1. Install PostgreSQL and create a database.
2. Set your database URL in `Rocket.toml`:

    ```toml
    [default.databases]
    blog_db = { url = "postgres://username:password@localhost/blog_db" }
    ```

3. Run Diesel migrations:

    ```sh
    diesel setup
    diesel migration run
    ```

## Running

```sh
cargo run
```

The API will be available at [http://localhost:8000](http://localhost:8000).

---

**Built with [Rocket](https://rocket.rs/) and [Diesel](https://diesel.rs/).**
