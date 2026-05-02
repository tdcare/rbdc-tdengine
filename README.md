# rbdc-tdengine

TDengine 数据库驱动 for [rbatis](https://github.com/rbatis/rbatis) / [rbdc](https://github.com/rbatis/rbdc)。

基于 [taos-connector-rust](https://github.com/taosdata/taos-connector-rust) 实现，通过 WebSocket 协议连接 TDengine。

## 版本兼容

| rbdc-tdengine | rbs  | rbdc | taos-connector |
|---------------|------|------|----------------|
| 4.9.0         | 4.8  | 4.9  | 0.12.3         |

## 特性 (Features)

| Feature            | 说明                          | 默认 |
|--------------------|-------------------------------|------|
| `ws`               | WebSocket 连接 (rustls)       | ✅   |
| `ws-tls-rustls`    | WebSocket + TLS (rustls)      |      |
| `ws-tls-native-tls`| WebSocket + TLS (native-tls)  |      |

## 安装

```toml
[dependencies]
rbatis = "4.9"
rbdc-tdengine = "4.9"
```

## 快速开始

```rust
use rbatis::RBatis;
use rbdc_tdengine::driver::TaosDriver;
use rbdc_tdengine::options::TaosConnectOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rb = RBatis::new();
    rb.init_opt(
        TaosDriver {},
        TaosConnectOptions {
            dsn: "taos+ws://localhost:6041/test".to_string(),
        },
    )?;

    // 执行查询
    let rows = rb.exec("CREATE TABLE IF NOT EXISTS meters \
        (ts TIMESTAMP, current FLOAT, voltage INT, phase FLOAT)",
        vec![],
    ).await?;

    Ok(())
}
```

### 使用连接池

```toml
[dependencies]
rbdc-pool-deadpool = "4.9"
```

```rust
use rbdc_pool_deadpool::DeadPool;

let pool = DeadPool::new(
    TaosDriver {},
    TaosConnectOptions {
        dsn: "taos+ws://localhost:6041/test".to_string(),
    },
    10, // 最大连接数
);
```

### 使用 rbatis CRUD 宏

```rust
use rbatis::crud;
use serde::{Serialize, Deserialize};
use rbdc::datetime::DateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Record {
    ts: DateTime,
    current: Option<f32>,
    voltage: Option<i32>,
    phase: Option<f32>,
}

crud!(Record {}, "meters");

// 插入
let record = Record {
    ts: DateTime::now(),
    current: Some(43.0),
    voltage: Some(44),
    phase: Some(45.0),
};
Record::insert(&mut rb, &record).await?;
```

## DSN 格式

```
taos+ws://<host>:<port>/<database>
```

示例：
```
taos+ws://localhost:6041/test
```

## License

MIT
