// trait本身有单独生命周期限制
trait Reader<'a> {
    fn read(&self) -> &'a str;
}

struct Text<'a> {
    content: &'a str,
}

impl<'a> Reader<'a> for Text<'a> {
    fn read(&self) -> &'a str {
        self.content
    }
}

// &dyn Reader<'a> 本身并没有显式指定生命周期，dyn Reader 是动态分发类型
// 原则上编译器无法从 reader 的上下文中推断出它的生命周期是否与 Text 的生命周期一致
// 但是Reader Trait 本身有 'a 生命周期限制，因此编译器会根据 Trait 的定义默认使用其显式生命周期
fn get_reader<'a>(reader: &dyn Reader<'a>) -> &'a str {
    reader.read()
}

fn main() {
    let text = Text {
        content: "Hello, Rust!",
    };

    let result = get_reader(&text);
    println!("{}", result);
}