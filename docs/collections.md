
## 1 array
### 1.1 创建一个空数组
```rust
fn main() {
    let arr:[i32; 0] = [];
}
```

### 1.2 创建一个指定长度的数组
```rust
fn main() {
    // 创建一个长度为 5 的数组，所有元素初始化为 0
    let array1: [i32; 5] = [0; 5];
    // 输出: [0, 0, 0, 0, 0]
    println!("{:?}", array1);

    // 方法 2：指定每个元素的值
    let array2: [i32; 3] = [1, 2, 3];
    // 输出: [1, 2, 3]
    println!("{:?}", array2); 
}
```

### 1.3 动态生成（适合可变数组）
如果数组长度需要动态指定，通常需要使用 Vec 而不是固定大小的数组:  
```rust
fn main() {
    let mut vec = vec![0; 5]; // 创建一个长度为 5 的动态数组，所有元素为 0
    vec.push(10); // 添加新元素
    println!("{:?}", vec); // 输出: [0, 0, 0, 0, 0, 10]
}
```

## 2 vector
### 2.1 创建一个长度为 0 的 vector
使用 Vec::new 或 vec![] 来创建一个空的 Vec:  
```rust
fn main() {
    let empty_vec: Vec<i32> = Vec::new(); // 显式指定类型
    println!("{:?}", empty_vec); // 输出: []

    let empty_vec2 = vec![]; // 类型会被推断
    println!("{:?}", empty_vec2); // 输出: []
}
```

### 2.2 创建一个指定长度的 vector
使用 vec![value; length] 创建一个指定长度并用默认值填充的 Vec:  
```rust
fn main() {
    let vec_with_length = vec![0; 5]; // 创建一个长度为 5 的 Vec，所有元素为 0
    println!("{:?}", vec_with_length); // 输出: [0, 0, 0, 0, 0]

    // 如果需要其他默认值：
    let vec_with_length2 = vec![42; 3]; // 长度为 3，所有元素为 42
    println!("{:?}", vec_with_length2); // 输出: [42, 42, 42]
}
```

### 2.3 创建一个指定元素的 vector
通过 vec![] 明确地列出元素来创建一个 Vec:  
```rust
fn main() {
    let vec_with_elements = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec_with_elements); // 输出: [1, 2, 3, 4, 5]
}
```

### 2.4 遍历vector
#### 2.4.1 遍历不可变引用
```rust
fn main() {
    let vec = vec![1,2,3,4];

    for item in &vec {
        // 因为 println! 宏内部调用了 Display 或 Debug，这些 trait 实现了对引用类型的支持
        // 因此item在这里不需要解引用
        println!("Item: {}", item);
    }
}
```

#### 2.4.2 遍历可变引用
```rust
fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    
    for item in list.iter_mut() {
        // 因为这里是对item进行计算，因此需要解引用出真实的值
        *item *= 2; // 将每个元素乘以 2
        println!("{:?}", vec); // 输出: [2, 4, 6, 8, 10]
    }
}
```

#### 2.4.3 遍历所有权（移动元素）
通过 into_iter 遍历所有权（会消耗 Vec），适用于需要获取所有权或销毁 Vec 的场景:  
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.into_iter() {
        println!("Item: {}", item);
    }
    // vec所有权此时 vec 已被消耗，无法再使用
}
```

#### 2.4.4 使用索引遍历
如果需要同时访问索引和元素，可以使用 enumerate 方法:  
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    for (index, item) in vec.iter().enumerate() {
        println!("Index: {}, Item: {}", index, item);
    }
}
```

#### 2.4.5 使用函数式迭代器方法
**示例 1：过滤元素:**  
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // filter接收的是一个&T, 那么T本身就是一个&i32的引用，即这里是&&i32
    // rust提供了语法糖 |&可以解引用| 
    let filtered: Vec<_> = vec.iter().filter(|&&x| x > 3).collect();
    // let filtered: Vec<_> = vec.iter().filter(|x| **x > 3).collect(); 和上面等价的
    println!("{:?}", filtered); // 输出: [4, 5]
}
```

**示例 2：映射元素:**  
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // 输出: [2, 4, 6, 8, 10]
}
```

## 3 hashmap
### 3.1 创建hashmap
### 3.1.1 使用库创建
```rust
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new(); // 创建一个空的 HashMap
    println!("{:?}", map); // 输出: {}
}
```

### 3.1.2 使用 collect 创建
```rust
use std::collections::HashMap;

fn main() {
    let map: HashMap<&str, i32> = [("key1", 10), ("key2", 20)].iter().cloned().collect();
    println!("{:?}", map); // 输出: {"key1": 10, "key2": 20}
}
```

### 3.1.3 使用第三方crate创建
通过 maplit 的 hashmap! 宏快速创建:  
```toml
[dependencies]
maplit = "1.0"
```

```rust
use maplit::hashmap;

fn main() {
    let map = hashmap! {
        "key1" => 10,
        "key2" => 20,
    };
    println!("{:?}", map); // 输出: {"key1": 10, "key2": 20}
}
```

### 3.2 插入键值对
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    println!("{:?}", map); // 输出: {"key1": 10, "key2": 20}
}
```

### 3.3 获取数据
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);

    if let Some(value) = map.get("key1") {
        println!("Value for key1: {}", value); // 输出: Value for key1: 10
    }
}
```

### 3.4 是否存在某一个key
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 10);

    if map.contains_key("key1") {
        println!("Key exists!");
    }
}
```

### 3.5 删除key
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.remove("key1"); // 删除键 "key1"
    println!("{:?}", map); // 输出: {}
}
```

### 3.6 遍历键值对
#### 遍历键值对
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    map.insert("key3", 30);

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
```

#### 遍历key
```rust
fn main() {
    for key in map.keys() {
        println!("Key: {}", key);
    }
}
```

#### 遍历value
```rust
fn main() {
    for value in map.values() {
        println!("Value: {}", value);
    }
}
```

## 4 tuple
### 4.1 创建一个简单元组
```rust
fn main() {
    let tuple = (1, "hello", 3.14); // 元组包含三个元素，类型分别为 i32、&str 和 f64
    println!("{:?}", tuple); // 输出: (1, "hello", 3.14)
}
```

### 4.2 创建嵌套元组
组可以嵌套，即一个元组可以包含另一个元组： 
```rust
fn main() {
    let nested_tuple = (1, (2, 3), 4);
    println!("{:?}", nested_tuple); // 输出: (1, (2, 3), 4)
}
```

### 4.3 访问元组的元素
#### 4.3.1 使用索引访问
可以通过 . 和索引访问元组的元素:  
```rust
fn main() {
    let tuple = (1, "hello", 3.14);
    println!("First: {}", tuple.0); // 输出: 1
    println!("Second: {}", tuple.1); // 输出: hello
    println!("Third: {}", tuple.2); // 输出: 3.14
}
```

#### 4.3.2 使用解构赋值
```rust
fn main() {
    let tuple = (1, "hello", 3.14);
    let (x, y, z) = tuple; // 解构元组
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

### 4.4 传递元组作为函数参数和返回值
#### 4.4.1 作为函数参数
```rust
fn print_tuple(t: (i32, &str, f64)) {
    println!("Tuple: {:?}", t);
}

fn main() {
    let tuple = (1, "hello", 3.14);
    print_tuple(tuple);
}
```

#### 4.4.2 作为函数返回值
```rust
fn create_tuple() -> (i32, f64) {
    (42, 3.14)
}

fn main() {
    let tuple = create_tuple();
    println!("{:?}", tuple); // 输出: (42, 3.14)
}
```