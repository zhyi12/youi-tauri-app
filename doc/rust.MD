# rust安装
    // linux macos
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # windows
    https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

# rust管理工具-rustup
## rustup命令

    //升级
    rustup update
    //指定安装版本
    rustup toolchain install nightly-2023-11-01
    // 安装最新稳定版本
    rustup update stable
    // 查看本机安装的版本
    rustup toolchain list
    // 切换使用版本
    rustup default stable/nightly/beta
    rustup default nightly-2023-10-25
    
## rustup 换源
    echo "export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static" >> ~/.bashrc
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.sjtug.sjtu.edu.cn/rust-static/rustup" >> ~/.bashrc
    source ~/.bashrc

# cargo 换源

    cat $HOME/.cargo/config
    [source.crates-io]
    registry = "https://github.com/rust-lang/crates.io-index"
    # 指定镜像
    replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc
    
    # 注：以下源配置一个即可，无需全部
    # 中国科学技术大学
    [source.ustc]
    registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    
    # 上海交通大学
    [source.sjtu]
    registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
    
    # 清华大学
    [source.tuna]
    registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
    
    # rustcc社区
    [source.rustcc]
    registry = "https://code.aliyun.com/rustcc/crates.io-index.git"