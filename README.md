# 桌面应用
    基于tauri构建的桌面应用。
## 一、UI
    基于 svelte kit/tailwindcss/daysiui 框架
<ul>
    <li>svelte.config.js</li>
    <li>postcss.config.cjs</li>
    <li>tailwind.config.js</li>
    <li>vite.config.ts</li>
</ul>

## 二、后台
    使用rust语言，基于tauri框架开发

### rust项目 
<li>框架代码 packages/youi</li>
    主项目不启用rust trust，文件数量太多，容易卡顿， 可以IDE单独导入youi项目并启用rust trust。
<li>服务命令 packages/command</li>
    业务需要的 tauri::command 方法
<li>tauri插件 packages/plugins</li>
    自定义的tauri插件

### 核心依赖

#### 框架
* [tauri](https://tauri.app/)
* [svelte](https://svelte.dev/)
* [svelte-kit](https://kit.svelte.dev/)
* [tailwind](https://tailwindcss.com/)
* [daisyui](https://daisyui.com/)

#### 数据计算
* [polars](pola-rs.github.io/polars-book/user-guide/index.html)

#### 异步框架
* [tokio](https://tokio.rs/)

#### 序列化
* [serde](https://github.com/serde-rs/serde)
* [serde-json](https://github.com/serde-rs/json)

#### 脚本语言
* [rhai](https://github.com/rhaiscript/rhai)

#### 数据库连接
* [sqlx](https://github.com/launchbadge/sqlx)

#### 全文检索
* [tantivy](https://github.com/quickwit-oss/tantivy)
* [cang-jie](https://github.com/DCjanus/cang-jie)
* [jieba-rs](https://github.com/messense/jieba-rs)

#### 地理信息
* [georust](https://github.com/georust/geo)
* [geojson](https://github.com/georust/geojson)

#### 机器学习
* [tract](https://github.com/sonos/tract)
* [smartcore](https://github.com/smartcorelib/smartcore)
* [candle](https://github.com/huggingface/candle)

# 运行
## tauri-cli
    cargo install tauri-cli
## 开发模式
    cargo tauri dev
## 打包模式
    cargo tauri build

