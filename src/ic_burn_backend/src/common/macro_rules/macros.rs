

#[macro_export]
macro_rules! stable_insert {
    ($mem: expr,$key: expr,$data: expr) => {
        $mem.with(|map| {
            let mut ref_mut = map.borrow_mut();
            ref_mut.insert($key,$data)
        })
    };
}

#[macro_export]
macro_rules! stable_get {
     ($mem: expr,$key: expr) => {
         let memory_data=$mem.with(|map| {
             map.borrow_mut().get($key)
         })
    };
}




