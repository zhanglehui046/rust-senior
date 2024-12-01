Rust 使用 包（Package） 和 模块（Module） 来组织代码，帮助开发者构建复杂的项目。通过这些机制，Rust 提供了清晰的代码结构、模块化的功能划分以及良好的可维护性。  
## 一 什么是Workspace
工作空间指的就是可能是存放多个项目的一个地方

## 二 包(Package)
可以理解为可以提供一系列功能的一个大包，它可以包含多个模块和二进制文件。包的主要特点:
一个包包含一个 Cargo.toml 文件，用于管理包的元数据(如名称、版本、依赖等)
一个包可以包含多个 crate，例如: 
    一个二进制 crate（通常是 src/main.rs）
    一个库 crate（通常是 src/lib.rs）
一个Package中Crate主要包含两种类型: Binary和Library
每一个Package可以有多个Binary Crate，但是最多只能有1个Library Crate
但是至少要包含一个Crate, 要么是Binary Crate，要么是Library Crate
每一个Package下有一个默认可执行文件main.rs
如果在bin中还定义了其他可执行文件，那么可以通过cargo run --bin 文件来指定

## 三 模块(Module)
模块 是 Rust 中用于组织代码的主要方式。模块可以嵌套，用于将代码分组或者分层，从而实现代码的封装、复用和访问控制  
模块可以定义在：  
    单独的文件中（通过 mod 声明并链接到文件）
    当前文件的 mod 块中
如果不是 pub，那么只能在同一模块中访问  
如果是 pub，那么可以在其他模块中访问  
可以在同一源代码文件中定义多个模块:  
```rust
mod english {
    pub mod greetings { /* ... */ }
    pub mod farewells { /* ... */ }
}
mod chinese {
    pub mod greetings { /* ... */ }
    pub mod farewells { /* ... */ }
}
```

也可以把模块定义成单独的文件，比如了library
lib.rs:  
mod english;  
mod chinese;

也可以用目录来组织模块：
lib.rs:  
```rust
mod english;
```

english/  
mod.rs:  
pub mod greetings;  
greetings.rs:  
当访问模块的成员时，默认的名字空间相对于当前模块，即当前模块不需要use就可以直接使用,当然你也可以选择使用绝对路径
```rust
mod one {
    mod two { pub fn foo() {} }
    fn bar() {
        two::foo()
    }
}
```

可以使用 pub use 来重新导出其他模块中的项目：  
```rust
#[cfg(english)]  
pub use english::*;  
#[cfg(chinese)]  
pub use chinese::*;
```

对于一个(library crate) 来说，在顶层模块 (root module) 中导出的项目是该相对外
暴露的内容。 也就是在 lib.rs 中有 pub 标注的项目。  

可以使用 self 或 super 来指定相对于当前模块的名字
```rust
use self::greetings;
use super::chinese;
```

## 四 什么是Crate
* 第一: Crate是Rust中编译的最小单位，或者叫做单元包，类似于Java中jar
* 第二: 每一个Crate都是由模块构成的一棵树
* 第三: 一个Package中Crate主要包含两种类型: Binary和Library
使用自己定义的crate:
```toml
[dependencies]
myfoo = { git = "https://github.com/me/foo-rs" }
mybar = { path = "../rust-bar" }
# 或者：
[dependencies.myfoo]
git = "https://github.com/me/foo-rs"
```
然后就可以使用了：
use myfoo::english;  


## 五 实践
5.1 如何正确设计模块结构
```css
my_project
├── Cargo.toml
└── src
    ├── main.rs
    ├── network
    │   ├── mod.rs
    │   ├── server.rs
    │   └── client.rs
    └── utils
        ├── mod.rs
        └── logger.rs

```
network/mod.rs:  
```rust
pub mod server; // 声明子模块 server
pub mod client; // 声明子模块 client

pub fn connect() {
    println!("Connecting to network...");
}
```

network/server.rs:  
```rust
pub fn start_server() {
    println!("Server started!");
}
```

network/client.rs:  
```rust
pub fn start_client() {
    println!("Client started!");
}
```

main.rs:  
```rust
mod network; // 声明网络模块
mod utils;   // 声明工具模块

fn main() {
    network::connect();
    network::server::start_server();
    network::client::start_client();
}
```

### 5.2 如何正确使用 pub use？
开发者不清楚：  
    是否应该直接暴露内部模块？
    如何简化模块的访问路径？
解决方案:  
通过 pub use 将内部模块或成员重新导出，简化使用者的访问路径  
```rust
mod network {
    pub mod server {
        pub fn start_server() {
            println!("Server started!");
        }
    }

    pub mod client {
        pub fn start_client() {
            println!("Client started!");
        }
    }

    // 简化外部访问路径
    pub use server::start_server;
    pub use client::start_client;
}

fn main() {
    // 直接调用顶层暴露的接口
    network::start_server();
    network::start_client();
}
```

### 5.3 如何拆分大型模块？
问题：  
当一个模块文件过大时，如何拆分模块但仍保持模块的逻辑一致性？

解决方案：  
    将模块拆分为多个文件
    使用 mod.rs 或同名目录作为模块的入口点
示例：  
模块拆分为目录结构：  
```css
my_project
└── src
    ├── main.rs
    └── network
        ├── mod.rs
        ├── server.rs
        └── client.rs

```

### 5.4 如何处理模块循环依赖？
问题：  
模块 A 需要引用模块 B，而模块 B 又需要引用模块 A，如何处理？  

解决方案：  
    重构代码：抽取公共逻辑到第三方模块
    延迟初始化：通过函数而不是直接引用解决
### 5.5 如何在库和二进制中共享模块？
问题：
当一个项目既有库（lib.rs）又有二进制（main.rs）时，如何共享模块？

解决方案：
将共享逻辑放在 lib.rs 中，通过二进制文件调用库。

示例：
lib.rs:  
```rust
pub mod utils {
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
}
```

main.rs:  
```rust
use my_project::utils;

fn main() {
    utils::greet("Alice");
}
```

### 5.6 模块初始化和静态变量的使用？
问题:  
模块中如何实现全局初始化和静态数据管理？

解决方案:  
使用 lazy_static 或 once_cell 实现惰性初始化  

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<String> = Mutex::new(String::from("Default Config"));
}

fn main() {
    let mut config = CONFIG.lock().unwrap();
    *config = String::from("Updated Config");
    println!("Config: {}", *config);
}
```





