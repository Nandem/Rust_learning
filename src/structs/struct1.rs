pub fn run()
{
    /*
    let user1 = User
        {
            email: String::from("test@test.test"),
            username: String::from("test"),
            active: true,
            sign_in_count: 1,
        };
    let user2 = User
        {
            email: String::from("test@test.test"),
            username: String::from("test"),
            ..user1
        };
    println!("{}{}", user1.username, user2.username);
    */
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn create_a_user(email: String, username: String) -> User
{
    User
    {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    length: u32,
}
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.length > other.length && self.width > other.width
    }
}
fn area(rect: &Rectangle) -> u32
{
    rect.length * rect.width
}