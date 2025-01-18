### Lp Browser



### 目录结构介绍

```
├── Cargo.lock
├── Cargo.toml
├── README.md               # reade me
├── config.toml             # 配置文件
├── crates
│   ├── apis                # 核心
│   ├── commons             # 核心
│   ├── cores               # 核心
│   ├── models              # 核心
│   ├── servers             # api服务
│   ├── services-remote     # api服务
│   ├── states              # api服务
│   └── views               # tauri
├── front                   # 前端代码
│   ├── README.md
│   ├── dist
│   ├── env.d.ts
│   ├── index.html
│   ├── node_modules
│   ├── package-lock.json
│   ├── package.json
│   ├── pnpm-lock.yaml
│   ├── public
│   ├── src
│   ├── text.txt
│   ├── tsconfig.app.json
│   ├── tsconfig.json
│   ├── tsconfig.node.json
│   └── vite.config.ts
├── migrations        
```

### Commands

任何命令在项目根目录下执行.

#### Dev Commands

- 执行api服务: `cargo run --bin apis`

- 启动客户端程序: `cargo tauri dev`