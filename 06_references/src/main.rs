fn main() {
    /* One way to return ownersip
    from functions would be to use tuples  */
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_t(s1);
    println!("The length of '{}' is {}.", s2, len);

    /* Ptr references allow us to conveniently avoid
    moving ownership out of the current scope.
    The function in this case "borrows" s1, but never owns
    it. */
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /* We need mutable references if we want to change
    a borrowed value */
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    /* Mutable references have one big restriction:
    if you have a mutable reference to a value,
    you can have no other references to that value.
    e.g,

    ```
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    ```

    will result in a compilation error.

    Thus, r1 would need to be dropped or moved
    before r2 can be used. This restriction
    prevents data race conditions.

    We can use curly brackets to create a new scope,
    allowing for multiple mutable references,
    just not simultaneous ones:
    */
    let mut _s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    /*
    We also cannot have a mutable reference while
    we have an immutable one to the same value.

    ```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    ```

    If the last usage of the immutable reference, 
    however, occurs before the mutable reference is introduced, 
    the code will compile 
    */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn calculate_length_t(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
