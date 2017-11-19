pub fn base_if() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

pub fn base_loop() {
    let mut loop_index = 0;
    loop {
        loop_index = loop_index + 1;
        println!("第{}次循环", loop_index);
        if loop_index == 100
        {
            break;
        }
    }
}

pub fn base_while() {
    let a = [11, 22, 33, 44, 55];
    let mut index = 0;

    while index < 5 {
        println!("测试数组的值a[{}]为：{}", index, a[index]);
        index = index + 1;
    }
}

pub fn base_for() {
    let a = ["aa", "bb", "cc", "dd", "ee"];
    let mut index = 0;

    for av in a.iter()
    {
        println!("测试数组的值a[{}]为：{}", index, av);
        index = index + 1;
    }
}

pub fn range_for() {
    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("LIFTOFF!!!");
}