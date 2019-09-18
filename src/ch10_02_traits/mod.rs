use std::fmt::{Debug, Display};

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    author: String,
    content: String,
    headline: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

struct Tweet {
    content: String,
    reply: bool,
    retweet: bool,
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only Pair's that implement Display and PartialOrder will have the cmp_display method
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

// Blanket implementation
// impl<T: Display> ToString for T {}

pub fn run() {
    let tweet = Tweet {
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
        username: String::from("horse_ebooks"),
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
    };

    println!("New article available! {}", article.summarize());

    // Trais as parameters
    notify(article);
    notify(tweet);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);
}

fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
fn notify_trait_bound<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2(item1: impl Summary, item2: impl Summary) {}
// Above vs below
fn notify_trait_bound2<T: Summary>(item1: T, item2: T) {}

// Multiple trait bound
fn notify_multi(item: impl Summary + Display) {}
fn notify_multi2<T: Summary + Display>(item: T) {}

// More clear code with `where` syntax
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    0
}
fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Returning trait implementations
fn returns_summarizable() -> impl Summary {
    Tweet {
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
        username: String::from("horse_ebooks"),
    }
}

// Only works when returning a single type, below won't compile
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//             username: String::from("horse_ebooks"),
//         }
//     } else {
//         NewsArticle {
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
//             ),
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//         }
//     }
// }
