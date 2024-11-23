## 1 What's the ownership?
中文:  
在 Rust 中，所有权是管理程序内存的一种独特机制。它决定了内存中数据的归属（即谁拥有数据）以及数据的生命周期。Rust 通过所有权规则确保内存安全，同时避免了运行时垃圾回收的需要。  
Rust 的所有权模型主要基于以下三个核心概念：  
所有者：一个变量是某个数据的唯一所有者。  
借用：可以通过引用（& 或 &mut）借用数据，而不改变所有权。  
作用域：当变量离开其作用域时，所有权随之转移或数据被释放。  

英文:  
In Rust, ownership is a unique mechanism for managing memory in programs. It determines who owns data in memory and governs its lifecycle. Rust uses ownership rules to ensure memory safety without relying on runtime garbage collection.  

The ownership model revolves around three core concepts:  

Owner: A variable that is the sole owner of some data.  
Borrowing: You can borrow data through references (& or &mut) without transferring ownership.  
Scope: When a variable goes out of scope, its ownership ends, and the associated memory is released or transferred.  


## 2 The effect of ownership?
中文:  
**内存安全**： 所有权模型通过编译时检查，防止常见的内存问题，如空指针引用、悬挂指针、内存泄漏等。  
**无垃圾回收**： Rust 不依赖运行时垃圾回收器来管理内存，而是**_通过所有权规则和作用域自动释放内存_**，提供更高的性能。  
**数据竞争防止**： 所有权结合 Rust 的借用规则和所有权转移机制，避免了并发编程中的数据竞争问题。  

英文:    
**Memory Safety**: The ownership model ensures memory safety by preventing common issues like null pointer dereferences, dangling pointers, and memory leaks through compile-time checks.  
**No Garbage Collection**: Rust manages memory without relying on runtime garbage collection. Instead, it uses ownership rules and scope to automatically free memory, providing better performance.  
**Preventing Data Races**: Ownership, combined with borrowing rules and transfer of ownership, prevents data races in concurrent programming.  

## 3 The ownership rules
中文：  
**所有权规则：**
每个值有且只有一个所有者（Owner）。  
当所有者离开作用域时，值会被自动释放（drop）。  
通过不可变引用（&T）或可变引用（&mut T）可以借用数据，但在同一作用域中，不可同时存在可变引用和不可变引用。  
如果是拷贝，所有权不会被转移，比如函数参数是基本数据类型，基本数据类型是值拷贝，因此所有权不会被转移。  

**借用规则：**  
任何时候只能有一个可变引用，或者任意多个不可变引用。  
可变引用和不可变引用不能同时存在。  

英文：  
**Ownership rules：**
Each value has exactly one owner.  
When the owner goes out of scope, the value is automatically dropped (freed).  
Data can be borrowed using immutable references (&T) or mutable references (&mut T), but mutable and immutable references cannot coexist in the same scope.  
If it is a copy, ownership will not be transferred. For example, if a function parameter is a primitive data type, the primitive type is passed by value, so ownership is not transferred.  

**Borrowing rules:**  
At any given time, you can have either one mutable reference or any number of immutable references.  
Mutable references and immutable references cannot coexist. 

```rust
fn main() {
    let s = String::from("hello"); // s owns the data
    takes_ownership(s);            // Ownership of s is moved to the function
    // println!("{}", s);          // ❌ Error: s is no longer valid after ownership is moved

    let x = 5;                     // x is an integer, a Copy type
    makes_copy(x);                 // x's value is copied
    println!("{}", x);             // ✅ x is still valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope, and its value is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer is a Copy type, so the original value is unaffected

```

## 4 Borrow
借用是通过引用（& 或 &mut）访问数据而不转移其所有权。以下是一些常见场景下的借用示例：  
### 4.1 Immutable Borrowing (&T)(不可变借用)
不可变借用允许对数据的只读访问，数据的所有者和生命周期保持不变。  
Immutable borrowing allows read-only access to data without affecting its ownership or lifetime.  
```rust
fn print_message(message: &String) {
    println!("Message: {}", message);
}

fn main() {
    let msg = String::from("Hello, Rust!");
    print_message(&msg); // Immutable borrow of msg
    println!("Original message: {}", msg); // msg is still valid
}

```
### 4.2 Mutable Borrowing(&mut T)(可变借用)
可变借用允许对数据进行修改，但同一时间只能有一个可变借用。即我借用你的数据，我还能修改你的数据，但是数据还是你的，你还是数据的所有者。  
Mutable borrowing allows modification of the data but only one mutable reference can exist at a time.  
```rust
fn append_message(message: &mut String) {
    message.push_str(" Welcome!");
}

fn main() {
    let mut msg = String::from("Hello, Rust!");
    append_message(&mut msg); // Mutable borrow of msg
    println!("Updated message: {}", msg); // msg has been modified
}
```
### 4.3 Multiple Borrowing in the Same Scope(同一作用域中多个借用)
Rust 不允许在同一作用域中同时存在可变借用和不可变借用，以下示例会报错:
Rust prevents simultaneous mutable and immutable borrowing within the same scope :
```rust
fn main() {
    let mut msg = String::from("Hello");
    let r1 = &msg; // Immutable borrow
    let r2 = &mut msg; // ❌ Error: Cannot borrow as mutable while immutable borrow exists
    println!("{}, {}", r1, r2);
}
```
Correct Example:  
Separate the lifetimes of immutable and mutable references:  
```rust
fn main() {
    let mut msg = String::from("Hello");

    // Immutable borrow
    let r1 = &msg;
    println!("Read-only: {}", r1);

    // Mutable borrow
    let r2 = &mut msg;
    r2.push_str(", Rust!");
    println!("Modified: {}", r2);
}
```
### 4.4 Returning References from Functions(函数返回引用)
函数不能返回局部变量的引用，因为局部变量会在函数结束时被销毁。
Functions cannot return references to local variables because the data would be dropped when the function exits.
Incorrect Example:
```rust
fn return_reference() -> &String { // ❌ Error: Cannot return a reference to a local variable
    let msg = String::from("Hello");
    &msg // msg is dropped after this function ends
}
```

Correct Example:  
Returning references to input arguments:  
```rust
fn return_reference(input: &String) -> &String {
    input // Return reference to input
}

fn main() {
    let msg = String::from("Hello, Rust!");
    let borrowed_msg = return_reference(&msg);
    println!("{}", borrowed_msg); // Output: Hello, Rust!
}
```
### 4.5 Borrowing in Collections(集合中的借用)
在迭代集合时，借用每个元素。  
Borrowing elements from a collection like a vector during iteration.  
Example: Borrowing elements in a collection  
```rust
fn main() {
    let vec = vec![1, 2, 3, 4];

    for v in &vec { // Immutable borrow of elements
        println!("Value: {}", v);
    }

    println!("Original vector: {:?}", vec); // vec remains valid
}

```

### 4.6 切片借用
借用数组或字符串的一部分时会使用切片（&[T] 或 &str）  
Borrowing parts of a string or array as slices (&str or &[T]).  
```rust
fn print_slice(slice: &str) {
    println!("Slice: {}", slice);
}

fn main() {
    let s = String::from("Hello, Rust!");
    print_slice(&s[0..5]); // Borrow a portion of the string
    println!("Original: {}", s); // s remains valid
}
```

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4];
    let result = sum(&arr[1..3]); // Borrow a portion of the array
    println!("Sum: {}", result); // Output: 5
}

```
### 4.7 Scoped Mutable Borrowing(可变借用的多步操作)
Rust 确保可变借用是安全的。
Rust ensures that mutable borrowing is valid only within its scope.  
Example: Modifying borrowed data  
```rust
fn main() {
    let mut data = vec![1, 2, 3];

    {
        let d = &mut data; // Mutable borrow
        d.push(4); // Modify data
    } // Borrow ends here

    println!("Modified data: {:?}", data); // Output: [1, 2, 3, 4]
}
```



### 4.8 Dynamic Borrowing (RefCell and Interior Mutability)(动态借用[RefCell 和内部可变性])
在某些情况下，Rust 使用 RefCell 允许在运行时检查借用规则，适用于共享数据的动态修改.  
RefCell allows dynamic borrow checking at runtime for shared data modification.  
```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(String::from("Hello"));

    // Dynamically borrow immutably
    let borrowed_data = data.borrow();
    println!("Read-only: {}", borrowed_data);
    drop(borrowed_data); // Manually end the borrow

    // Dynamically borrow mutably
    let mut borrowed_data_mut = data.borrow_mut();
    borrowed_data_mut.push_str(", Rust!");
    println!("Modified: {}", borrowed_data_mut);
}
```