# WeChatSaver


## Dependencies
- FFmpeg
- sqlcipher
 - 编译的时候，要加上环境变量`RUSTFLAGS="-L /opt/homebrew/opt/sqlcipher/lib"`
```shell
# 依赖
rusqlite = {version = "0.32.1",features = ["sqlcipher"]}
# 编译的时候，要加上环境变量
 export RUSTFLAGS="-L /opt/homebrew/opt/sqlcipher/lib"
```