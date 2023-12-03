use std::fmt::{Debug, Display};

pub trait Summary {
    // No default - must have impl for type
    // fn summarize(&self) -> String;

    // Default implementations can call other methods 
    // in the same trait, even if those other methods don’t 
    // have a default implementation.
    fn summarize_author(&self) -> String; 


    // Default
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Hello {}", self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {  
    // accepts any type that implements the Summary trait
    // we can call any methods on item that come from the Summary trait
    println!("Breaking news! {}", item.summarize());
}

// Full trait bound syntax - equivalent to above 
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


// Multiple trait bounds with + syntax 
// pub fn notify(item: &(impl Summary + Display)) { } 
// Or 
// pub fn notify<T: Summary + Display>(item: &T) { }


// Where clauses for clearer trait bounds 
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }
// Or 
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 1 }


// Return type implements trait 
fn returns_summarizable() -> impl Summary {
    // can only return one type - in this case, cannot conditionaaly return NewsArticle
    // even though it also implements Summary
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


// Trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // only implements the 
    // cmp_display method if its inner type T implements the PartialOrd trait 
    // that enables comparison and the Display trait that enables printing
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}