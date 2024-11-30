### HUBSTUDIO & ADSPOWER BROWSER

环境管理页面包含的选项:

- 新建环境
- 批量导入
- 打开环境
- 环境项
  - 详情
  - 编辑环境
  - 编辑账号
  - 修改代理
  - 编辑 cookie
  - 授权成员
  - cookierobot
  - 转移环境
  - 清除缓存
  - 运维授权
  - 设为常用
  - 删除环境
- 关闭环境
  - 关闭已选
  - 关闭所有
- 更多
  - 批量更新环境
  - 批量更新 cookie
  - 批量导出
  - 分组管理
- 刷新
- 窗口排列
- 搜索
  - 环境
  - 序号
  - 环境编号
  - IP
  - 账号
  - 账号名称
  - 备注
- 筛选

  - ...

- 最近打开
- 环境分组
- 回收站
- 代理视角
- 团队扩展
- 我的账号
- API

1. 无头模式相关参数
   参数 描述
   --headless 启用无头模式（没有图形界面）。适合用于服务器端运行或自动化测试。
   --disable-gpu 禁用 GPU。通常与 --headless 一起使用，提升兼容性。
   --no-sandbox 禁用沙盒模式，通常用于无头模式下，避免权限问题（注意安全风险）。
   --remote-debugging-port 启用远程调试端口，例如 --remote-debugging-port=9222。
   --disable-software-rasterizer 禁用软件光栅化，通常与无头模式一起使用。
   --disable-extensions 禁用所有扩展。
   --hide-scrollbars 隐藏滚动条，通常用于截图。
   --mute-audio 静音所有音频输出。
2. 性能优化参数
   参数 描述
   --disable-plugins 禁用所有插件（包括 Flash 等）。
   --disable-images 禁用图片加载，可提升性能。
   --disable-javascript 禁用 JavaScript 脚本执行。
   --blink-settings 配置 Blink 渲染引擎的行为，例如 --blink-settings=imagesEnabled=false。
   --disable-cache 禁用缓存，强制从网络加载内容。
   --disk-cache-size 设置磁盘缓存大小（字节），例如 --disk-cache-size=104857600（100MB）。
3. 网络相关参数
   参数 描述
   --proxy-server 设置代理服务器，例如 --proxy-server=socks5://127.0.0.1:1080。
   --proxy-bypass-list 设置不使用代理的地址列表，例如 `--proxy-bypass-list="\*.example.com;127.0.0.1"。
   --ignore-certificate-errors 忽略所有 SSL 证书错误（用于测试环境）。
   --user-agent 设置自定义的用户代理字符串，例如 --user-agent="CustomUserAgent"。
   --disable-web-security 禁用同源策略，允许跨域请求（用于开发和测试，不推荐生产环境使用）。
4. 界面控制参数
   参数 描述
   --start-maximized 启动时窗口最大化。
   --window-size 设置窗口尺寸，例如 --window-size=1920,1080。
   --window-position 设置窗口位置，例如 --window-position=0,0。
   --incognito 启动隐身模式（无痕模式）。
   --app 启动特定的 Web 应用程序，例如 --app=https://www.example.com。
   --kiosk 启动全屏模式（信息亭模式）。
5. 开发和调试参数
   参数 描述
   --enable-logging 启用日志记录，通常与 --v 参数一起使用以设置日志级别，例如 --v=1。
   --v 设置日志详细级别，--v=1 为较低级别，--v=99 为最高详细级别。
   --enable-remote-extensions 启用通过远程加载的扩展。
   --devtools 启动时打开开发者工具（DevTools）。
   --remote-debugging-address 设置远程调试的绑定地址，例如 --remote-debugging-address=0.0.0.0。
6. 安全相关参数
   参数 描述
   --disable-web-security 禁用浏览器的 Web 安全机制，例如同源策略（仅用于测试环境）。
   --no-default-browser-check 禁用默认浏览器检查提示。
   --allow-running-insecure-content 允许加载不安全的内容（如 HTTP 内容）。
   --disable-features 禁用某些浏览器特性，例如 --disable-features=IsolateOrigins,site-per-process。
7. 多媒体相关参数
   参数 描述
   --mute-audio 静音所有音频输出。
   --autoplay-policy 设置自动播放策略，例如 --autoplay-policy=no-user-gesture-required。
   --enable-features 启用特定的功能，例如 --enable-features=NetworkService,SharedArrayBuffer。

### 开发计划

11.24-11.26

- 核心代码封装

  该项开发计划包含桌面客户端的各种操作，比如创建环境、管理环境、分组、批量打开等提供用户操作的环境核心功能和方法。

11-27-12.02

- 编写供外部调用 API

  这里要开发两套逻辑，但都是对核心代码的方法的调用。

  - 分别是编写 API 后端让用户可以通过请求直接来操作环境等核心功能(该条包含的功能指应用中的功能)。
  - 编写供前端调用的命令让用户可以通过鼠标点击等操作来操作环境等核心功能(该条包含的功能指应用中的功能)。

12-03-12.05

- 最后构建部分

  最后负责和前端对接、整个应用构建、不同系统兼容性处理等上线前要完成的内容。

注意，上述项完成过程中可能会增加功能、完善功能、调整功能等所以时间会差个 1-2 天。且在步骤二完成后要取得前端代码。

```
4个浏览器

C:\Users\cgy>tasklist | findStr chrome
chrome.exe                    3272 Console                    1    124,736 K
chrome.exe                   14888 Console                    1      8,792 K
chrome.exe                    6828 Console                    1    123,772 K
chrome.exe                    8276 Console                    1     30,756 K
chrome.exe                   15732 Console                    1     18,908 K
chrome.exe                   13440 Console                    1     48,132 K
chrome.exe                   15292 Console                    1     48,044 K
chrome.exe                   16968 Console                    1     47,260 K
chrome.exe                    9964 Console                    1     47,488 K
chrome.exe                   13652 Console                    1     55,104 K
chrome.exe                   11140 Console                    1     26,828 K

memory: 180(该数据来自任务管理器，并不精确)。
```

```
4个浏览器

C:\Users\cgy>tasklist | findStr chrome
chrome.exe                   10804 Console                    1    106,428 K
chrome.exe                    5656 Console                    1     96,496 K
chrome.exe                   15112 Console                    1     96,408 K
chrome.exe                    8080 Console                    1     96,188 K
chrome.exe                    9244 Console                    1      8,864 K
chrome.exe                   15376 Console                    1      8,888 K
chrome.exe                    6588 Console                    1      8,740 K
chrome.exe                    3548 Console                    1      8,904 K
chrome.exe                   16532 Console                    1     90,648 K
chrome.exe                   17328 Console                    1     28,884 K
chrome.exe                   16804 Console                    1    100,616 K
chrome.exe                   12332 Console                    1     28,732 K
chrome.exe                   15832 Console                    1     96,608 K
chrome.exe                    3204 Console                    1     18,256 K
chrome.exe                   16216 Console                    1     28,828 K
chrome.exe                    8304 Console                    1     86,248 K
chrome.exe                   12776 Console                    1     18,224 K
chrome.exe                   16484 Console                    1     18,248 K
chrome.exe                     216 Console                    1     28,848 K
chrome.exe                    5984 Console                    1     18,292 K
chrome.exe                    6916 Console                    1     57,964 K
chrome.exe                   13280 Console                    1     55,760 K
chrome.exe                   13408 Console                    1     59,116 K
chrome.exe                   18044 Console                    1     58,848 K
chrome.exe                   18984 Console                    1     28,820 K
chrome.exe                   19028 Console                    1     26,644 K
chrome.exe                   19048 Console                    1     28,812 K
chrome.exe                   19092 Console                    1     26,640 K

memory: 400(该数据来自任务管理器，并不精确)。
```

<!-- playwright-rust
thirtyfour
fantoccini
webbrowser-rs
webdriver
 -->