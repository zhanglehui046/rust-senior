## 1 Functions(函数)
### 1.1 Normal Function(普通函数)
#### 1.1.1 Function Definition(函数定义)
```rust
fn <function name> (parameter1: type1,parameter2: type2,......) -> <return type> {
    return .....
}
```
If one value or expression in the last line, the return keyword could be ignored or omitted.
```rust
fn <function name> (parameter1: type1,parameter2: type2,......) -> <return type> {
    if (condition) {
        return .......
    } else if (condition) {
        return .......
    }
    xxxxx or a+b
}
```

### 1.2 Advanced Function(高级函数)
#### 1.2.1 Generic Function(泛型函数)
特点: 泛型函数允许你定义一个函数，该函数可以接受任意类型的参数，并在函数体内使用该类型，而无需进行强制类型转换.  
##### 1.2.1.1 Generic Function Definition(泛型函数定义)
```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if (a > b) {
        return a
    }
    b
}

fn main() {
    println!("Max: {}", max(10, 20)); // Output: 20
    println!("Max: {}", max(1.5, 2.3)); // Output: 2.3
}
```
##### 1.2.1.2 Generic Constraint(泛型约束)
可以通过约束限制泛型参数必须实现某些特性（Traits）。常用语法包括 <T: Trait> 和 where 子句。
You can constrain generic parameters to types that implement specific traits. Common syntax includes <T: Trait> and where clauses.  

**单一约束 | Single Constraint**:  
```rust
fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
```

**多个约束 | Multiple Constraints**:  
```rust
fn print_and_clone<T: Clone + std::fmt::Debug>(value: T) {
    let cloned = value.clone();
    println!("{:?}", cloned);
}
```

**where 子句 | Using where Clause**:  
```rust
fn complex<T, U>(x: T, y: U)
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("{:?}, {}", x.clone(), y);
}

```


#### 1.2.2 Anonymous Function(匿名函数)
##### 1.2.2.1 Anonymous Function Future(特性)
中文:  
* 匿名函数是没有名字的函数  
* 不能捕获外部环境变量  
* 适合作为函数指针, 比如传递给其他函数  

英文:
* Anonymous functions are unnamed functions
* Anonymous functions cannot capture variables from the surrounding environment, unlike closures.  
* Anonymous functions are often used where function pointers (fn type) are required, such as implementing traits or passing to other functions.  

##### 1.2.2.2 Anonymous Function Usage Scene(使用场景)
中文:
匿名匿名函数常用于以下场景：
* 高阶函数的参数  
```rust
fn bar(func: fn(i32,i32) -> i32, x: i32, y: i32) -> i32 {
    func(x,y)
}

fn foo<T: PartialOrd, F: Fn(T, T) -> T>(func: F, x: T, y: T) -> T {
    func(x,y)
}


fn inner(a: i32, b:i32) -> i32 {
    a * b
}

fn main() {
    let result = foo(inner, 5, 6);
    println!("Result: {}", result)
}
```
* 事件处理或回调函数  
```rust
fn handle(callback: fn()) {
    println!("Event triggered!");
    callback(); // Call the provided anonymous function
}

fn handle_event<F>(callback: F)
where
    F: Fn(),
{
    println!("Event triggered!");
    callback(); // Call the provided anonymous function
}

fn main() {
    handle_event(|| println!("Handling event in callback!")); // Output: Event triggered! Handling event in callback!
    handle(|| println!("Handling event in callback!"));
}
```

* 一次性临时逻辑  
```rust
fn main() {
    let x = 5;
    // Anonymous function used for one-time square calculation
    // |y| captures surrounding variable x
    let result = (|y| y * y)(x); 
    println!("{}", result); // Output: 25
}
```

* 捕获外部变量的逻辑  
```rust
fn main() {
    let multiplier = 2;
    // multiply不是一个结果，而是一个闭包变量，而是可以执行的函数
    // multiply is not a result, but rather a closure variable
    let multiply = |x| x * multiplier; // 捕获 multiplier 的所有权
    println!("{}", multiply(5)); // 输出: 10
}
```

* 延迟执行或动态行为生成  
```rust
fn main() {
    let deferred = || println!("Deferred execution!");
    println!("Before deferred execution");
    deferred(); // Output: Before deferred execution \n Deferred execution!
} 
```
**闭包在这里的意义是延迟执行**：  
deferred 是一个闭包，定义时并不会立即执行，只有在需要时通过 deferred() 调用闭包，才会运行其内部逻辑。  
这种模式常用于动态逻辑，或者需要在未来某个特定时机执行的代码。  

**适用场景**：  
这段代码中，闭包被用于延迟执行的逻辑，可以应用于以下场景：
*     事件驱动：在特定事件发生时才执行逻辑。
*     动态行为：根据运行时需求调整闭包逻辑。
*     调试代码：在某些流程中插入延迟打印或调试信息。


* 多线程任务  
```rust
fn main() {
    let handle = thread::spawn(|| {
        println!("Running in a separate thread!");
    });
    handle.join().unwrap(); // Wait for the thread to finish
}
```

* 配置、测试和调试逻辑  
```rust
fn main() {
    let debug = |x| {
        println!("Debug: {}", x);
        x + 1
    };

    println!("{}", debug(10)); // Output: Debug: 10 \n 11
}

```

* 链式调用和组合函数  
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let result: Vec<_> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0) // Filter even numbers
        .map(|x| x * x)         // Square them
        .collect();
    println!("{:?}", result); // Output: [4, 16]
}
```
英文:
Anonymous functions are commonly used in the following scenarios:
* Parameters for higher-order functions
* Event handling or callback functions
* One-time temporary logic
* Logic that captures external variables
* Deferred execution or dynamic behavior generation
* Multi-threaded tasks
* Configuration, testing, and debugging logic
* Chained calls and function composition


#### 1.2.3 Async Function(异步函数)
##### 1.2.3.1 Async Function Future(特性)
中文:  
* 异步函数是通过async关键字修饰的函数
* 异步函数并不会立即执行，而是返回一个实现了Future Trait的值
* 最后通过.await()来等待执行结果

英文:
* In Rust, an async function is marked with the async keyword  
* These functions isn't execute immediately, but return a value that implements the Future trait  
* Invoke the await() to get the execution result, it will wait until the function done.


```rust
// 异步函数的定义语法
async fn foo() -> i32 {
    42
}
// 调用异步函数
#[tokio::main]
async fn main() {
    let result = foo().await;
    println!("Result: {}", result);
}
```

##### 1.2.3.2 Async Function Effect(异步函数的作用)
中文:  
Rust 的异步函数通过非阻塞方式处理任务，允许单个线程在多个任务之间切换，从而提高性能。它们通常用于：  
* I/O 操作（如文件、网络请求）：等待外部资源时避免阻塞线程  
* 任务并发处理：在单线程环境中高效地管理多个任务
* 事件驱动程序：如 GUI 或实时系统的事件循环

英文:
Async functions enable non-blocking task execution, allowing a single thread to switch between tasks efficiently. They are particularly useful for:  

* I/O operations (e.g., file or network requests): Avoid blocking threads while waiting for external resources.
* Concurrent task execution: Efficiently handle multiple tasks in a single-threaded environment.
* Event-driven programming: Ideal for GUI or real-time systems with event loops.

##### 1.2.3.3 Async Function Usage Scene(使用场景)
**Network Requests(网络请求):**  
Perfect for making HTTP requests or other network calls  
异步函数非常适合发起 HTTP 请求或其他网络调用  
```rust
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://example.com").await?.text().await?;
    Ok(response)
}

```

**File Operations(文件操作):**
Reading or writing files without blocking the main thread  
读取或写入文件时可以避免阻塞主线程  
```rust
use tokio::fs;

async fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}
```

**Concurrent Tasks(并发任务):**  
Use tokio::join! or futures::join! to run multiple tasks simultaneously  
使用 tokio::join! 或 futures::join! 同时运行多个异步任务  
```rust
use tokio;

async fn task_one() {
    println!("Task one started");
}

async fn task_two() {
    println!("Task two started");
}

#[tokio::main]
async fn main() {
    tokio::join!(task_one(), task_two());
}
```

##### 1.2.3.4 Challenges and Pitfalls(挑战和坑)
**难点一:** 异步函数返回的是Future, 并不是一个实际值，需要通过调用.await获取结果，在这个过程中会挂起当前线程，等待任务完成。  
**Challenges 1:** Async functions return a Future, not the actual result，You must use .await to get the result。  
```rust
async fn example() -> i32 {
    42
}

fn main() {
    // ❌ Error: Cannot directly call async functions
    let result = example();
}
```
**难点二:** 异步函数如果接收的参数是一个外部引用，不能直接返回引用。
**Challenges 2:** Can not return the external reference if async function parameter is a external reference  
```rust
async fn example<'a>(data: &'a str) -> &'a str {
    data // ❌ 不允许返回引用
}
```

**为什么异步函数不能直接返回引用？**
**生命周期问题**
```rust
async fn example<'a>(data: &'a String) -> &'a str {
    println!("Step 1: Received data: {}", data);
    sleep(Duration::from_secs(1)).await; // ❌ 问题：任务暂停，外部任务再继续执行，可能外部引用已经失效
    &data[0..5] // 试图返回部分引用
}
```

**编译器的限制**
异步函数会返回一个 Future，其生命周期是 'static，这要求返回值不依赖外部引用，除非你明确告诉编译器如何管理生命周期。  

**如何使用 async move 确定生命周期？**
* async move是用来捕获异步函数环境中的数据，并将这些数据的所有权转移到异步任务中。  
* 通过 move，异步函数可以确保所有捕获的数据在任务生命周期内有效，因为数据的所有权已经转移到异步任务中。

```rust
// 异步函数 example 接受一个 String 类型的参数 data,它的返回值是一个 Future，最终会产生一个 String 类型的值
async fn example(data: String) -> String {
    // async move 创建了一个新的异步块，将 data 的所有权移动到异步块中
    async move { // 使用 `async move` 明确生命周期
        format!("Processed: {}", data) // 使用了 data，并产生一个新的 String
    }
    .await // 异步块返回的 Future 被 .await 调用并完成。最终返回的是异步块中计算的值，即 format!("Processed: {}", data) 生成的字符串。
}
```


**异步函数执行可能跨越多个 await 点，如何在代码中体现？**
代码示例：await 跨越多个暂停点:  
```rust
use tokio::time::{sleep, Duration};

async fn example(data: String) -> usize {
    println!("Step 1: Received data: {}", data);
    sleep(Duration::from_secs(1)).await; // 第一个 await 点

    println!("Step 2: Processing data...");
    sleep(Duration::from_secs(1)).await; // 第二个 await 点

    println!("Step 3: Done processing!");
    data.len()
}
```
任务逻辑：  
第一个 await：模拟等待 1 秒，例如等待网络请求。  
第二个 await：再次等待 1 秒，例如处理另一个任务。  
每个 await 都是一个暂停点，异步任务会暂停，允许其他任务运行。  
重要性：  
在 await 点暂停时，Future 仍然持有 data，必须保证 data 在任务恢复时仍然有效。  

**难点三:** 难以调试  
**Challenges 3:** Difficult to debug

**难点四:** 并发时的共享数据，异步任务之间共享状态需要小心，例如使用 Arc<Mutex<T>> 来保证线程安全  
**Challenges 4:** Shared data in concurrent tasks
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let data = shared_data.clone();
        handles.push(tokio::spawn(async move {
            let mut lock = data.lock().await;
            *lock += 1;
        }));
    }

    for handle in handles {
        handle.await
    }
}
```
#### 1.2.4 High-Order Function(高阶函数)
**特点**: 可以接收一个函数作为参数，也可以返回一个函数作为返回值
##### 1.2.4.1 Function as Parameter(函数作为参数)
```rust
fn apply<F>(operation: F, a: i32, b: i32) -> i32
where
    F: Fn(i32, i32) -> i32, // 接受一个函数 F，签名为 Fn(i32, i32) -> i32
{
    operation(a, b) // 执行传入的函数
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = apply(add, 5, 3); // 将函数 add 作为参数传递
    println!("5 + 3 = {}", result); // 输出: 5 + 3 = 8

    let result = apply(|x, y| x * y, 4, 6); // 传递匿名函数
    println!("4 * 6 = {}", result); // 输出: 4 * 6 = 24
}

```

##### 1.2.4.2 Return Value is Function(返回值是函数)
```rust
// 这里的 impl Fn(i32) -> i32 表示返回一个类型，
// 这个类型实现了 Fn(i32) -> i32 trait，也就是说这个返回值是一个闭包，接收一个 i32 参数并返回一个 i32。
// Rust 的闭包是匿名类型，无法直接在函数签名中明确写出闭包的类型，因此需要一种机制来表达“返回某种符合某个特定 trait 的类型”。impl Trait 允许你指定返回值符合某个 trait，而不需要关心其具体类型
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    // move 将 factor 的所有权移动到闭包中，使闭包能够在其生命周期内安全地持有
    move |x| x * factor // 返回一个闭包，捕获 factor
    // print!("factor: {}", factor); 编译报错, factor所有权已经转移到闭包内了
}

fn main() {
    let multiply_by_2 = create_multiplier(2); // 创建一个乘以 2 的闭包
    let multiply_by_5 = create_multiplier(5); // 创建一个乘以 5 的闭包

    println!("2 * 3 = {}", multiply_by_2(3)); // 输出: 2 * 3 = 6
    println!("5 * 4 = {}", multiply_by_5(4)); // 输出: 5 * 4 = 20
}

```

## 2 Closure Package(闭包)
### 2.1 What's the closure(什么是闭包)?
**闭包**是一个能够捕获其定义环境中的变量的**匿名函数**,可以访问其作用域外的变量。  
A closure is an anonymous function that can capture variables from its defining environment. Unlike regular functions, closures can access variables outside their own scope and capture them by reference, mutable reference, or by value as needed.  

### 2.2 Closure Definition(闭包定义)
闭包的语法使用 |参数| 表达式，并且可以是多行代码块:
Closures are defined using the syntax |parameters| expression, and they can also be multi-line blocks:  
基本形式:  
```rust
let closure = |x| x + 1;
```
多行形式:  
```rust
let closure = |x| {
    println!("Input: {}", x);
    x + 1
};

```

### 2.3 Features of Closures(闭包的特性)
中文:  
**捕获环境变量**： 闭包可以访问和捕获其定义环境中的变量。捕获方式包括：
* 按引用捕获：闭包读取外部变量
* 按可变引用捕获：闭包修改外部变量
* 按值捕获：闭包获取外部变量的所有权
**类型推断**： Rust 编译器可以根据上下文推断闭包参数和返回值的类型
**实现 Trait**： 闭包自动实现了 Fn、FnMut 或 FnOnce，根据捕获方式确定。

英文:  
**Capturing Environment Variables**: Closures can access and capture variables from their defining environment in the following ways:
* By Reference (default): The closure reads the external variable
* By Mutable Reference: The closure modifies the external variable
* By Value: The closure takes ownership of the external variable
**Type Inference**: The Rust compiler infers the types of closure parameters and return values based on context
**Implements Traits**: Closures automatically implement Fn, FnMut, or FnOnce, depending on the capture method

### 2.4 Use Cases of Closures(闭包用途)
中文: 
**函数式编程**：通过高阶函数（如 map、filter）传递逻辑
```rust
let nums = vec![1, 2, 3];
let squares: Vec<_> = nums.iter().map(|x| x * x).collect();
println!("{:?}", squares); // 输出: [1, 4, 9]
```

**事件处理**：闭包可以作为回调函数处理用户输入或系统事件
**延迟计算**：将逻辑封装在闭包中，并在需要时执行
```rust
let deferred = || println!("Deferred execution!");
println!("Before execution");
deferred(); // Output: Before execution \n Deferred execution!
```

**并发编程**：闭包常用于多线程或异步任务中，携带任务逻辑
```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Running in a separate thread!");
});

handle.join().unwrap();
```

英文:  
* **Functional Programming**: Passing logic through higher-order functions like map and filter
* **Event Handling**: Closures act as callback functions to handle user input or system events
* **Lazy Evaluation**: Wrapping logic in a closure and executing it when needed
* **Concurrent Programming**: Used in multithreading or async tasks to encapsulate log


### 2.5 Challenges and Difficulties of Closures(闭包的挑战和难点)
中文:
**捕获方式**:  
Rust 自动选择捕获方式（引用、可变引用或值），可能会引发编译错误，需要显式调整  
解决方案：使用 move 强制按值捕获  

**闭包的生命周期**：
闭包可能引用了生命周期较短的变量，导致编译器拒绝通过  
解决方案：通过 move 或调整生命周期  

**性能问题**：
闭包捕获变量时可能有额外的性能开销，尤其是复杂的捕获场景  

**多线程场景**：
闭包在多线程中需要 Send 和 Sync，否则无法在线程间传递  

英文:  
**Capture Modes**:
Rust automatically chooses the capture mode (reference, mutable reference, or value), which can cause compilation errors when unexpected  
Solution: Use move to force value capture  

**Closure Lifetimes**:  
Closures may reference variables with shorter lifetimes, leading to compiler errors  
Solution: Use move or adjust lifetimes  


**Performance Concerns**:
Capturing variables in closures can introduce overhead, especially in complex scenarios  

**Multithreading**:
Closures need to be Send and Sync to work across threads, which can be restrictive  