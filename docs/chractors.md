### 1 string
Here are two string types, one is str type, and the other is String type.  

#### 1.1 str &str
str is a core string type in rust, representing an array or sequence of UTF-8 encoded bytes.  
str is a dynamically sized type, meaning its size unknown in compile time.  
str is immutable, so we can not use it directly， and modifying it is even more prohibited.
because of this, we can read or reference it with &str type.

#### 1.2 String
String is a core string type in rust, representing a dynamic length bytes array.
String is allowed to be modified, we can push some strings to it.  
By default, a String is immutable unless we use the mut keyword to make the variable mutable  
A String is allocated on the heap, and consists of three important parts: ptr、usize and capacity. The capacity has initialized size, and as long as the data length not exceed to the capacity, data can continue to be written into the current memory. If it exceeds the capacity, the memory will be resized, The current values are then copied to the new memory, and the previous memory is released.  

#### 1.3 str &str vs String
str literal content is stored in the read-only memory or static memory area, &str is stored in the stack; String is stored in the heap  
str has a compile-time unknown length but a fixed runtime length; String has a dynamically adjustable length at runtime  
&str is a immutable reference, can not be allowed be modified by mut; By default, a String is immutable unless we use the mut keyword to make the variable mutable， we can invoke push or push_str change the content  
```rust
fn main() {
    let s1: &str = "hello"; // A string slice pointing to a string literal
    let s2: String = String::from("world"); // A heap-allocated String
    let s3: &str = &s2; // A slice of the String, referencing its content

    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // s3 is a &str pointing to s2's content
    // s1: hello, s2: world, s3: world
}
```
#### 1.4 usage scene
"&str" is used for more 'read-only' operations. Since it's an immutable reference, it does not own the underlying memory and cannot alter it.
"String" is used when you need to own a string and change its contents, like appending characters or changing a character at a certain position.
"&mut str" allows you to perform some in-place modifications to a string, as long as you maintain valid UTF-8. However, its use is less common and usually requires more care.
&mut str is a reference to a mutable str, but such cases are very rare because str is an immutable type. The only scenario where &mut str might be involved is when performing mutable operations on a heap-allocated Box<str> or similar structures via slicing.  
```rust
fn main() {
    let mut s: Box<str> = "hello".into();
    let s_mut: &mut str = &mut s;
    s_mut.make_ascii_uppercase(); // Convert the content to uppercase
    println!("{}", s_mut); // Output: HELLO
}
```
### 2 character
char represents one character, such as 'a'、'1'
declare one char array:

```rust
fn main() {
    let chars: [char; 4] = ['a', 'b', 'c', 'd']; // declare a char array contains 4 characters
    println!("{:?}", chars);
}
```