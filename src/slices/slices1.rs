pub fn run()
{
    let mut s = String::from("hello word");
    let word = first_word(&s);

//    s.clear();

    //println!("s length is {}", word);
    println!("s's first word is {}", first_word_slice(&s));
}

fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
        {
            if item == b' '
            {
                return &s[0..i];
            }
        }
    &s[..]
}