fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    assert_eq!(v1_iter.next(), Some(&1)); // Note that we needed to make v1_iter mutable: calling the
                                          // next method on an iterator changes internal state that the
                                          // iterator uses to keep track of where it is in the sequence.
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Iterator adaptor, e.g., map()
    // does not consume the iterator
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect consumes the iterator
                                                         // and collects the resulting values into a 
                                                         // collection data type

    assert_eq!(v2, vec![2, 3, 4]);

    // Using closures that capture their environment
    

}
