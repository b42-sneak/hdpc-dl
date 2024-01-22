# hdpc-dl

A downloader for HDPC.

```zsh
docker create --pull always --name hdpc_db -p 8000:8000 surrealdb/surrealdb:latest start --auth --user root --pass sKDSm1bHejzy0x38orxYG8sxWZg6wYNAlncEC4HTpWWJ8iOpNf file:/home/nonroot/data/mydatabase.db
```

`-v` doesn't work, because this db is a fucking joke
