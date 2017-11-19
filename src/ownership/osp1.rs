//    //报错，s1所有权被移动至函数内，外部不在有效
//    let s1 = String::from("s1");
//    osp1::string_ownership(s1);
//    println!("{}", s1);
//
//    //可以正常运行，s2所有权被返回给s3
//    let s2 = String::from("s2");
//    let s22 = osp1::string_ownership_with_return(s2);
//    println!("{}", s22);
//
//    //可以正常运行，s3的引用被传递给函数
//    let s3 = String::from("s3");
//    osp1::string_ownership_reference(&s3);
//    println!("{}", s3);

pub fn base_ownership1() {
    let mut  string = "string";
    string = "new string";
    println!("{}", string);
}

pub fn string_ownership(s: String) {
    println!("值为{}的变量不在可用", s);
}

pub fn string_ownership_with_return(s: String) -> String {
    println!("值为{}的变量仍然可用", s);
    s
}

pub fn string_ownership_reference(s: &String) {
    println!("值为{}的变量仍然可用", s);
}

pub fn string_ownership_borrow_mut(s: &mut String) {
    s.push_str(" world");
}

pub fn string_ownership_reference_limited() {
    let mut s = String::from("s");

//    let r1 = &s;
//    let r2 = &s;
//    let r3 = &mut s;
}