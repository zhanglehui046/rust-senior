
struct Draw;
fn bar1<'a, 'b>(symbol: &'b &'a Draw) -> &'a Draw {
    *symbol // 解引用是&'a Draw，返回的生命周期也是&'a Draw, 因此没问题
}

// 报错: ^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
// fn bar2<'a, 'b>(symbol: &'b &'a mut Draw) -> &'a Draw {
//     *symbol // 解引用是&'a mut Draw，要求返回的是&'a Draw类型，要返回的生命周期作用域大于或者等于解引用后的生命周期，对于可变引用是不允许
// }
//
// fn bar3<'a, 'b>(symbol: &'b mut &'a Draw) -> &'a Draw {
//     *symbol // 解引用是&'a Draw，但是要求返回&'a Draw，这里是不可变引用，因此这里没问题
// }
//  ^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
fn bar4<'a, 'b>(symbol: &'b mut &'a mut Draw) -> &'a Draw {
    *symbol // *symbol // 解引用是&'a mut Draw，要求返回的是&'a Draw类型，要返回的生命周期作用域大于或者等于解引用后的生命周期，对于可变引用是不允许
}


fn main() {

}