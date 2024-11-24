## 1 What's the Trait(什么是Trait)
**Trait**是定义一系列行为的规范，只有函数签名没有函数体 对于实现trait的对象，必须全部实现trait定义的所有函数 trait本质上和其他语言的接口类似，特别是go语言  
**Trait** is a specification that defines a series of behaviors, containing only function signatures without function bodies. For objects that implement a trait, all functions defined in the trait must be implemented. Essentially, traits are similar to interfaces in other languages, particularly in Go.  

## 2 Trait Definition(Trait定义)
**impl {trait} for 对象 {...}**
要实现trait的对象必须是在本地crate定义的；无法为外部类型实现外部trait
trait定义的时候可以有默认实现，对象实现trait如果默认的实现满足需求，哭可以不用实现该函数

To implement a trait for an object, the object must be defined in the local crate; you cannot implement an external trait for an external type。  
When defining a trait, default implementations can be provided. If the default implementation meets the requirements, the object implementing the trait does not need to provide its own implementation for that function  

```rust
pub trait Action {
    fn say(&self, word: String);

    fn run(&self);

    fn food_type(&self) -> String;

    fn defaults() {
        println!("trait default function implements")
    }
}


pub struct Animal {
    pub types: AnimalType,
    pub name: String,
    pub age: u32
}

pub enum AnimalType {
    Tiger,
    Sheep,
    Cat,
    Dog
}

impl Action for Animal {

    fn say(&self, word: String) {
        println!("{} says: {}", self.name, word);
    }

    fn run(&self) {
        println!("{}'s {} is running", self.age, self.name);
    }

    fn food_type(&self) -> String {
        match self.types {
            AnimalType::Tiger => String::from("Meat"),
            AnimalType::Sheep => String::from("Grass"),
            AnimalType::Cat => String::from("Fish"),
            AnimalType::Dog => String::from("Bone")
        }
    }
}
```

## 3 Trait as Parameter(trait可以作为参数)
```rust
// trait作为参数
pub fn visit<T: Action>(item: T) {
    item.say(String::from("Hi, uncles and aunts!"));
    item.run();
    let food = item.food_type();
    println!("food -> {}", food)
}


// 第一种方案:
pub fn notify<T: Action + Display, E: Clone + Debug> (a: T, b: E) -> String {
    "notify".to_string()
}

// 采用第二种: where
pub fn inform<T,U> (a: T, b: U) -> String
where T: Action + Display, U: Clone + Debug {
    "inform".to_string()
}
```


## 2.4 Trait as Return Value(trait作为返回值)
trait作为返回值，函数中要返回的实现trait的类型必须保持一致，不能是因为类似于判断，可能返回实现相同trait的不同实现

```rust
fn getAnimal() -> impl Action {
    let cat = Animal{types:AnimalType::Cat, name: String::from("Jimmy"), age: 2};
    cat
}
```

## 2.5 为实现了trait X 的任何T实现 trait Y
```rust
// 为实现了trait X的任何T实现trait Y
impl<T: Copy> Action for T {
    fn say(&self, word: String) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }

    fn food_type(&self) -> String {
        todo!()
    }
}
```