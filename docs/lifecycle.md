## 一 指针、引用和生命周期回顾
### 1.1 指针
#### 1.1.1 什么是指针
一个指针是一个变量，存储的是一个内存地址，可以通过该地址直接访问和操作数据

####  1.1.2 为什么需要指针?  
**直接操作内存,具有很好的灵活性**  
指针允许程序直接访问内存地址，从而具备极大的灵活性，尤其在系统编程和底层开发中非常重要。例如： 操作硬件设备时需要直接访问硬件寄存器的地址。

**提高性能**  
通过指针操作数据可以避免不必要的数据拷贝，从而提升性能。例如:  
    函数传参： 通过指针传递数据避免复制大型数据结构
    共享数据： 多个模块可以通过指针共享数据，而无需额外分配内存

**动态内存管理**  
指针是动态内存分配的基础。它允许程序在运行时根据需要分配和释放内存。例如:  
    在堆（heap）上分配内存块并通过指针操作这些内存
    实现动态数组、链表和其他动态数据结构

#### 1.1.3 指针存在哪些问题
**问题一: 悬垂指针(Dangling Pointer)**  
**问题描述:**  
    悬垂指针是指指针指向了一块已经被释放或无效的内存地址
    访问悬垂指针会导致未定义行为，可能引发程序崩溃或数据泄漏
**发生场景:**  
    指向了超出作用域的栈变量
    指向了被释放的堆内存
```
#include <stdio.h>
#include <stdlib.h>

int* create_data() {
    int x = 42; // 栈上的局部变量
    return &x;  // 返回指向局部变量的指针
}

int main() {
    int *p = create_data(); // p 指向已经释放的地址
    printf("%d\n", *p);     // 未定义行为
    return 0;
}
```
**问题二: 野指针(Wild Pointer)**   
**问题描述:**  
    野指针是指未经初始化的指针，指向未知或随机内存位置
    访问野指针可能导致崩溃或破坏程序状态
**发生场景:**  
    指针在声明后未被初始化
    指针指向了非法的内存地址
```
#include <stdio.h>

int main() {
    int *p;          // 未初始化的指针
    *p = 42;         // 未定义行为，可能导致程序崩溃
    printf("%d\n", *p);
    return 0;
}
```
**问题三: 内存泄漏(Memory Leak)**  
**问题描述:**  
    程序分配了内存但未正确释放，导致内存长期占用而无法回收
    内存泄漏会耗尽系统资源，导致程序性能下降或崩溃
**发生场景:**  
    使用动态内存分配（如 malloc/new）但忘记调用 free/delete
    指针被重写，导致无法访问原分配的内存
```
#include <stdlib.h>

int main() {
    int *p = (int *)malloc(sizeof(int));
    *p = 42;
    // 忘记释放 p 的内存
    return 0; // 内存泄漏
}

```
**问题四: 双重释放(Double Free)**  
**问题描述:**  
    程序尝试释放同一块内存多次，可能导致未定义行为或程序崩溃
**发生场景:**  
    动态分配内存后，指针被释放多次
    指针被错误地共享或重新分配
```
#include <stdlib.h>

int main() {
    int *p = (int *)malloc(sizeof(int));
    free(p);
    free(p); // 再次释放，未定义行为
    return 0;
}
```

#### 1.1.4 解决方案
**方案一: 考程序员的能力解决**  
依靠程序的经验来解决，但是人总会出现一些问题的  

**方案二: 程序语言通过引入GC**  
进行垃圾回收，但是存在性能和卡顿问题，比如java 和 python  

**方案三: rust引入所有权和生命周期解决，但是学习成本高、理解难度大**  

## 1.2 引用和生命周期
引用的目的是为了防止转移所有权，即实现不转移所有权的时候，可以进行参数拷贝等  
但是所有权是可能会释放掉的，而引用的生命周期如果长于所有权主体，那么就会带来问题，因此需要引用必须提前比主体消失掉  


## 二 生命周期
### 2.1 什么是生命周期
**生命周期:**  
    引用在作用域中有效的时间范围就是生命周期
    每个 Rust 的引用都有一个生命周期
    一般而言，生命周期是隐式的，由编译器推断

### 2.2 为什么需要生命周期
生命周期的主要目标是防止悬空引用（dangling references），即程序引用的数据已经不再有效  
```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }
    // r来源于x的引用，但是x生命周期在代码段值性能结束就结束了
    // 这里再使用这个引用，就会报错
    println!("r: {r}"); 
}
```
上述代码无法通过编译。错误信息如下:  
```
error[E0597]: `x` does not live long enough
--> src/main.rs:6:13
|
5 |         let x = 5;
|             - binding `x` declared here
6 |         r = &x;
|             ^^ borrowed value does not live long enough
7 |     }
|     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
|                  --- borrow later used here
```

## 三 借用检查器(The Borrow Checker)
### 3.1 什么是借用检查器
借用检查器是在编译时期，被用于检查变量所有权和借用规则是否被正确遵守
    它是属于编译器的一部分内容
    通过借用检查器确保内存安全，避免数据竞争(data race)、悬垂指针(dangling pointer)和其他常见的内存管理问题

### 3.2 借用检查器的主要职责
**所有权规则:** 一个值在任意时间内只能有一个可变引用(&mut)，或者任意数量的不可变引用(&)，但不能同时存在  
**生命周期:** 确保引用的生命周期不会超出其数据的生命周期，从而防止悬垂指针的出现  
**可变性:** 检查对可变引用的修改是否符合规则，并确保不会对数据的访问造成不一致性  
**备注:** 即使代码逻辑上可能存在悬空引用，但只要未使用，编译器会认为代码是安全的
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+

```
这里，我们用 'a 表示 r 的生命周期，用 'b 表示 x 的生命周期。可以看到，'b 的范围比 'a 短。编译器通过比较生命周期，发现引用 r 的生命周期比其指向的数据的生命周期更长，因此拒绝编译这段代码  

## 四 生命周期标注(Lifetime Annotation)
### 4.1 什么是生命周期标注
* 是一个用来描述 引用 在程序中存在时间的机制
* 在编译时验证引用不会超出其数据的生命周期
* Rust 使用生命周期标注来确保引用的安全性
* 生命周期标注可以显式标注和不显示标准，当Rust 无法明确推断生命周期，就需要显式标注
* 标注方式通过在借用符号&'{annotation}, 比如&'a
* **生命周期标注语义**: 值来源于某个参数或者多个参数有关系，返回的引用生命周期取决于这些参数的生命周期最短的一个

### 4.2 为什么需要生命周期标注
编译器可以明确推断出生命周期，是不需要显式标注的
当Rust 无法明确推断生命周期，就需要显式标注，明确告诉编译器返回的值和哪些参数或者字段有关系，或者说来源于哪些字段或者参数
举一个例子: 
这段代码，result = longest(&m, &n); result不知道是来自于&m还是&n，不知道和他们哪个有关系，因为这里没有借用表明关系  
```rust
fn longest(x: &str, y: &str) -> &str {
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

fn main() {
    let m = String::from("good");
    let result;
    {
        let n = String::from("beautiful");
        result = longest(&m, &n);
    }
    println!("result value: {}", result)
}
```

或者这种也是会编译报错的:  
因为返回值可能是x, 也可能是y, 如果只是对x进行显式生命周期标注，即你认为我只想result来源于x，和x有关系，但是实际返回值y也有可能会返回，因此也会报错**lifetime `'a` required**:  
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else {
        x
    }
}

fn main() {
    let m = String::from("beautiful");
    let result;
    {
        let n = String::from("good");
        result = longest(&m, &n);
    }
    println!("result value: {}", result)
}
```

```
error[E0621]: explicit lifetime required in the type of `y`
  --> lifetime.rs:23:9
   |
19 | fn foo<'a>(x: &'a str, y: &str) -> &'a str {
   |                           ---- help: add explicit lifetime `'a` to the type of `y`: `&'a str`
...
23 |         y
   |         ^ lifetime `'a` required

error: aborting due to 1 previous error; 2 warnings emitted

```

因此，正确的生命周期标注，应该是这样的:    
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

fn main() {
    let m = String::from("beautiful");
    let result;
    {
        let n = String::from("good");
        result = longest(&m, &n); // 取决于&m和&n参数中生命周期最小的生命周期
    }
    // 因此这里无法获取result, 因为result生命周期等同于n的生命周期，而n的生命周期在代码快结束就结束
    println!("result value: {}", result)
}
```

### 4.3 生命周期省略规则(什么时候不需要生命周期标注？)
**单一引用参数:**  
对于只有一个引用参数的函数:  
```rust
fn first_word(s: &str) -> &str {
    &s[..1]
}
```

**如果函数有多个引用参数，但其中一个是 &self 或 &mut self，返回值的生命周期与 self 的生命周期一致:**  
以下代码没有显式标注生命周期：  
```rust
struct Example;

impl Example {
    fn get_ref(&self, value: &str) -> &str {
        value
    }
}

```

编译器会推断等价于：  
```rust
impl Example {
    fn get_ref<'a>(&'a self, value: &'a str) -> &'a str {
        value
    }
}
```
在这种情况下，返回值的生命周期是与 &self 一致的，因为这是编译器的默认规则  

## 五 常见的需要使用生命周期标注场景
### 5.1 函数返回引用
最常见的场景是当函数返回引用时，生命周期标注用于表明返回值的生命周期与输入参数的生命周期之间的关系  
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 5.2 结构体包含引用
如果一个结构体包含引用类型的字段，就需要生命周期标注。因为结构体的生命周期需要与引用的生命周期绑定  
```rust
truct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    // novel 是一个 String 类型的值，它的生命周期从声明开始，一直持续到 main 函数结束，因为它是在 main 函数作用域中定义的
    // first_sentence 是一个字符串切片（&str），它引用了 novel 中的部分数据。它的生命周期受 novel 的生命周期限制，因此 first_sentence 的生命周期与 novel 的生命周期一致
    // excerpt.part 是一个字符串切片引用，引用的是 first_sentence，而 first_sentence 又引用了 novel 的数据。因此，excerpt.part 的生命周期最终与 novel 的生命周期绑定
    println!("Excerpt: {}", excerpt.part);
} 
```

### 5.3 方法中的生命周期
我们定义一个结构体 ImportantExcerpt，它包含一个引用 part，表示某段文本的一部分。我们为该结构体实现了几个方法，这些方法需要使用显式的生命周期标注来确保引用的安全性:
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 方法返回结构体中引用的字段
    fn get_part(&self) -> &str {
        // part生命周期和self绑定，即便&self没有显式添加注解，这是默认的
        self.part
    }

    // 方法接收另一个引用并返回一个较短生命周期的引用
    // 即这里取决于self和'b生命周期较短的那个
    fn compare_with<'b>(&self, other: &'b str) -> &'b str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };

    // 使用 get_part 方法
    println!("Excerpt part: {}", excerpt.get_part());

    // 使用 compare_with 方法
    let another = "A short sentence.";
    let longer = excerpt.compare_with(another);
    println!("The longer part is: {}", longer);
}
```

### 5.4 多重引用之间的关系
当函数或方法有多个引用参数时，生命周期标注用于明确这些引用之间的关系:  
```rust
fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

### 5.5 复杂的引用嵌套
在某些复杂的引用嵌套情况下，例如泛型类型中包含引用时，生命周期标注可能是必要的  
```rust
fn print_with_lifetime<'a, T>(x: &'a T)
where
    T: std::fmt::Display,
{
    println!("{}", x);
}
```

### 5.6 与 static 生命周期的场景
当需要声明引用的生命周期为 'static（即存活于程序整个运行周期）时，也需要使用生命周期标注  
```rust
fn static_lifetime_example() -> &'static str {
    "I have a static lifetime"
}
```

**什么是 'static 生命周期？**  
在 Rust 中，'static 生命周期表示:  
1. 数据在整个程序运行期间有效
2. 常见的'static生命周期场景:  
- 存储在程序的常量或静态存储区中的数据(如字面量字符串 "hello")
- 没有引用其他短生命周期数据的结构体或实现



