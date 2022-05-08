ベンチマークの起動
```shell
cargo bench -- fan_in:: # fan_in::とついたベンチマークの実行
```

docker
```shell
docker compose build --no-cache --pull # Dockerfile(image)をrebuild
docker compose run --rm testcases_rust /bin/bash # dockerコンテナ内部でbash起動
```
