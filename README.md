# 校园网登录认证

------

## 适用范围

- 理论上适用于大部分锐捷认证的网络

## 使用预构建的二进制文件

### 运行

```bash
./single_login ./your_config
# your_config是配置文件名
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
```

### 交叉编译

```bash
cargo install cross
cross build --release --target mips-unknown-linux-musl
mips-linux-gnu-strip ./target/mips-unknown-linux-musl/release/single_login
```

------

## Systemctl设置自启动

```bash
sudo nano /lib/systemd/system/single_login.service
# 然后粘贴下面的内容并保存
```

```bash
# [Service]的ExecStart里的user_name请改为自己的用户名, your_cfg.cfg是配置文件名
[Unit]
Description=single_login
After=network.target syslog.target
Wants=network.target

[Service]
Type=simple
Restart=always
RestartSec=86400
ExecStartPre=/bin/sleep 20
ExecStart=~/single_login/single_login ~/single_login/your_cfg.cfg

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
```

