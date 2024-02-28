use std::fmt::Display;

fn main() {

    // ONE FUNCTION LARGEST TAKES PARAMETERS OF DIFFERENT TYPE. 
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest item in number_list is {}", largest(&number_list));
    println!("The largest item in char_list is {}", largest(&char_list));
    
    // GENERIC TYPES
    // Instance of the Tweet struct that implements Trait Summary.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, of course"),
        replay: false,
        retweet: false,
    };
    println!("new tweet: {}", tweet.summarize());

    // Instance of the NewsArticle struct that implements Trait Summary.
    let news = NewsArticle {
        headline: String::from("Vim is back"),
        location: String::from("Internet"),
        author: String::from("Tony Miv"),
        content: String::from("Vim there vim here"),
    };
    println!("News: {}", news.summarize());

    longer_of_two();
}

// FUNCTION THAT REMOVE DUPLICATION BY TAKING GENERIC TYPES AS PARAMTERS
// here we setup Trait Bound for Types that implement PartialOrd(this will 
// allow to use > operator) and Copy. 
// With the Trait bound defined this function will work multiple types that meet those two
// requirements
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Trait SUMMARY with one method SUMMARIZE_AUTHOR that must be defined by 
// everyone who implements it, and method SUMMARIZE that is avaliable to use 
// straight away, and custom implementation is optional!
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// TRAIT BOUNDS
// this function will take only those type who implements Trait Summary!
// Trait bounds are defined with the declaration of generic paramter T, and after colon inside 
// <angle brackets> | Literally here T: >Summary<, a Summary is a Trait Bound
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// You can specify multiple trait bounds on one function
// notice that you can have multiple parameters of generic type and each can have its own Trait
// Bound defined
// pub fn some_fuction<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> {};
// less verbose version
// fn some_function<T,U>(t: T, u: U) -> i32
//     where T: Dipslay + Clone,
//           U: Clone + Debug
// { 
// }
// STRUCT 
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// TRAIT implemented for STRUCT NewsArticle
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}
// STRUCT 
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub replay: bool,
    pub retweet: bool,
}
// TRAIT implemented for STRUCT Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
        
    }
}

// TRAIT BOUNDS TO CONDITIONALLY IMPLEMENTS METHODS
struct Pair<T> {
    x: T,
    y: T,
}

// this function will be implemented on every single instance of PAIR
impl<T> Pair<T> {
    //return object self
    fn _new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// Conditional implementation, only the instance of Pair that implements Display & PartialOrd
// will be able to use cmp_display method!
impl<T: Display + PartialOrd> Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is {}", self.x);
        } else {
            println!("The largest member is {}", self.y);
        }
    }
}

// VALIDATING REFERENCES WITH LIFETIMES
fn longer_of_two() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

