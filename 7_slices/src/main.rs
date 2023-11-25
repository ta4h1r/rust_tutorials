fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("{}", s.as_bytes()[word - 1] as char);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // Having to worry about the index in word getting
    // out of sync with the data in s is tedious and error prone

    // Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    println!("{}", slice1 == slice2); // true

    let s = String::from("hello");
    let len = s.len();
    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("{}", slice1 == slice2); // true

    // First word using slices
    let mut s = String::from("hello world");
    let _word = first_word_slice(&s); // Here we borrow s, then store an immutable ref to s
    s.clear(); // error! Trying to mutate s,
               // but word is an immutable reference to s
               // println!("the first word is: {}", _word); // This line and the one above it
               // cannot exist simultaneously. i.e., we cannot use an immutable reference after
               // the referenced value has been mutated

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

/*
Returns the index of the last letter of the
first word in a string separated by spaces
*/
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
