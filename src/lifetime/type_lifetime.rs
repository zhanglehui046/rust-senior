#[derive(Debug)]
pub struct SplitStr<'a> {
    start: &'a str, // 默认情况报错，缺少生命周期的指定: Missing lifetime specifier
    end: &'a str
}

pub fn split<'m, 'n>(text: &'m str, token: &'n str) -> Option<SplitStr<'m>> {
    let (start, end) = text.split_once(token)?;
    Some(SplitStr{start, end} )
}
