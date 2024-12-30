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


部分1：(12.31)
1. 所有表格对应上。
2. 消息提示(目前主要是接收到团队邀请).
3. 导航栏分组可以选择.
4. 尽可能多的找出bug和未完成的功能并记录。


部分2：(12.31)
1. 邀请成员, 加入团队 (邀请成员和加入团队放到一起去)
2. 切换团队
3. 消息提示(目前主要是接收到团队邀请).
4. 尽可能多的找出bug和未完成的功能并记录。


部分3：(12.31)
1. 编辑环境。
2. 环境详情。
3. 编辑账号。
4. 尽可能多的找出bug和未完成的功能并记录。


部分4：(12.31)
1. 所有分页有bug(修复及统一样式).
2. 编辑代理。
3. 转移环境。
4. 尽可能多的找出bug和未完成的功能并记录。
