## 1 Integer
### 1.1 无符号
u8 、u16 、u32 、u64、 u128、 usize
### 1.2 有符号
i8 、i16 、i32 、i64、 i128、 isize

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

## 2 bool
```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

## 3 char
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

## 4 Tuple
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

## 5 array
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // 声明数据类型和长度5
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 声明长度为5，数字3填满整个数组
    let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
}

fn declare() {
    // 第一种：声明类型和数量
    let arr1: [i32;5] = [1,2,3,4,5];

    // 第二种：不声明类型和数量
    let arr2 = [1,2,3,4,5];

    // 第三种：默认填充数组,长度为5，默认值全是2
    let arr3: [i32; 5] = [2;5];

    // 未初始化的数组（仅限unsafe代码）：
    unsafe {
        let arr4: [i32; 5] = MaybeUninit::uninit().assume_init();
    }
}

fn foreach() {
    let arr: [i32;5] = [1,2,3,4,5];

    // 第一种遍历方式 for in
    for element in arr {
        println!("for --> {}", element)
    }

    // 第二种遍历方式: 迭代器
    for data in arr.iter() {
        println!("iter --> {}", data);
    }

    // 第三种遍历，可以转换为索引和元组的遍历方式
    for (index, item) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, item);
    }

    // 第四种遍历：使用index遍历
    for i in 0..arr.len() {
        println!("Value at index {}: {}", i, arr[i]);
    }
}
```
#### 6 vector