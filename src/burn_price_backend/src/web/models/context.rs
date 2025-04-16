pub struct Context<T> {
    context: T,
    //暂时的 抽象的 公共的 字段  后续可能需要
    // ext1:String,
    // ext2:String,
    // ext3:String,
}

impl<T> Context<T> {
    pub fn new(c: T) -> Self{
        Context { context: c }
    }
    fn get_context<'a>(&'a self) -> &'a T {
        &self.context
    }
}
