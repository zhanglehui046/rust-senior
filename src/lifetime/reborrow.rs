struct Draw;
//
// // 参数是一个对不可变引用的不可变再借用
// fn foo1<'a, 'b>(symbol: &'b &'a Draw) -> &'b Draw {
//     // symbol: &'b &'a Draw
//     // 连续是不可变引用: 说明'a是'b的一个子类，即'a生命周期长于'b, 'b是依赖了'a的  => 'a : 'b
//     *symbol // 解引用是&'a Draw，但是要求返回一个短的引用，但是长的也没问题
// }
//
// // 参数是一个对可变引用的不可变再借用
// fn foo2<'a, 'b>(symbol: &'b &'a mut Draw) -> &'b Draw {
//     *symbol // &'a mut Draw, 一个可变引用可以转换成不可变引用
// }
//
// // 参数是一个对不可变引用的可变再借用
// // fn foo3<'a, 'b>(symbol: &'b &'a Draw) -> &'b mut Draw {
// //     // *symbol // &'a Draw, 一个不可变引用不可以转换成可变引用，所以会报错
// // }
//
// // 参数是一个对不可变引用的可变再借用
// fn foo4<'a, 'b>(symbol: &'b mut &'a Draw) -> &'b Draw {
//     *symbol // &'a Draw 长生命周期转换成短生命周期也是可以
// }
//
// // 参数是一个对可变引用的可变再借用
// fn foo5<'a, 'b>(symbol: &'b mut &'a mut Draw) -> &'b Draw {
//     *symbol // &'a mut Draw 一个可变引用可以转换成不可变引用
// }







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
// fn bar4<'a, 'b>(symbol: &'b mut &'a mut Draw) -> &'a Draw {
//     *symbol // *symbol // 解引用是&'a mut Draw，要求返回的是&'a Draw类型，要返回的生命周期作用域大于或者等于解引用后的生命周期，对于可变引用是不允许
// }

