# Zero to production book

## Docker

Prepare `sqlx` for offline mode by generating query metatdata to support offline compile-time verification:

```sh
cargo sqlx prepare -- --lib
```

Start the Docker daemon:

```sh
sudo service docker start
```

Build the Docker image:

```sh
docker build --tag zero2prod --file Dockerfile .
```

Run the image:

```sh
docker run -p 8000:8000 zero2prod
```

Note: If Docker makes your WSL image consume too much disk space, you can free it up first pruning:

```sh
docker image prune
docker prune
```

Then compacting unused space in the WSL image as described here:

- https://superuser.com/a/1612289
- https://github.com/microsoft/WSL/issues/4699#issuecomment-627133168

## Test

Requests:

curl -v http://127.0.0.1:8000/health_check

