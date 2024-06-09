# 校园网登录认证

------

## 适用范围

- 理论上适用于大部分锐捷认证的网络

## 使用预构建的二进制文件

现在提供`amd64`的`Windows`和`Linux`预构建文件、以及`aarch64`的`Linux`预构建文件，请在`release`里下载

### 运行

```bash
./single_login ./account.cfg
# account.cfg是配置文件名
```

### 配置文件格式应该如下

```bash
your_account_id	# 第一行为账号
your_password	# 第二行为密码
```

------

## 编译

### 安装rust环境

```bash
# amd64
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# arm64
wget https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init
chmod 700 rustup-init
./rustup-init
# git clone本仓库, 换rust镜像源
git clone https://github.com/mapkkkk/single_login.git
cd single_login
echo 'export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup' >> ~/.bash_profile
echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bash_profile
```

### 编译

```bash
cargo build --release
strip ./target/release/single_login
# 如果是windows，请按照下面的命令操作
strip .\target\release\single_login.exe
```

### 交叉编译(以`aarch64`为例)

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-musl
aarch64-linux-gnu-strip ./target/aarch64-unknown-linux-musl/release/single_login
```

------

## Linux下设置Systemctl设置自启动

```bash
sudo nano /lib/systemd/system/single_login.service
# 然后粘贴下面的内容并保存
```

```bash
# [Service]的ExecStart里的user_name请改为自己的用户名, account.cfg是配置文件名
[Unit]
Description=single_login
After=network.target syslog.target
Wants=network.target

[Service]
Type=simple
Restart=always
RestartSec=86400
ExecStartPre=/bin/sleep 20
ExecStart=~/single_login/single_login ~/single_login/account.cfg

[Install]
WantedBy=multi-user.target
```

```bash
# 设置自启动
sudo systemctl enable single_login
# 启动
sudo systemctl start single_login
# 查看运行状态(请在启动后过一会再查看)
sudo systemctl status single_login
# 停止运行
sudo systemctl stop single_login
# 重启
sudo systemctl restart single_login
```

------

## Windows下设置自启动

在`single_login`的文件夹里新建`single_login.bat`，替换里面的路径，将账号密码写入配置文件

```bash
@echo off
if "%1" == "h" goto begin
mshta vbscript:createobject("wscript.shell").run("""%~nx0"" h",0)(window.close)&&exit
:begin
REM
cd C:\single_login # 替换成single_login的文件夹路径
single_login account.cfg # account.cfg是配置文件
exit
```

创建`single_login.bat`的快捷方式，放入下面的路径的文件夹里

```powershell
C:\Users\rhett\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
```

------

`Mac`用户请自行`brew install`解决，本人无`Mac`，对`Mac`不熟
