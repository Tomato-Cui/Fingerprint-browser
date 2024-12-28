### HUBSTUDIO & ADSPOWER BROWSER

hubstudio & adspower browser

### 目录结构介绍

```
├── Cargo.lock
├── Cargo.toml
├── README.md         # reade me
├── clients              # 客户端可执行文件
├── config.toml       # 配置文件
├── crates
│   ├── cores         # 核心
│   ├── server        # api服务
│   └── views         # tauri
├── front             # 前端代码
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
├── migrations        # 数据库迁移文件
└── tests             # 测试代码
```

### 程序执行

```sh
git clone -b client https://github.com/Tu-guang/zonghu-fingerprint-frontend.git

# 执行前端代码
cd zonghu-fingerprint-frontend/frontend && npm i && npm run dev

# 执行客户端代码
cd zonghu-fingerprint-frontend/clients && cargo run --bin clients
```


1. 查询团队分组要有用户数量统计