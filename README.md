# 校园网登录认证

------

## 适用范围

- 理论上适用于所有锐捷认证的网络

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

