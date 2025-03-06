### Lp Browser

![微信截图_20250307073440](https://github.com/user-attachments/assets/1a725bb8-3c67-4b8a-958b-01b424c6e678)

![微信截图_20250307073609](https://github.com/user-attachments/assets/fdbc0179-1afc-4e7b-b0c4-a11b075e640e)


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


