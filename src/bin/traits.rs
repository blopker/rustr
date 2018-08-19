pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    fn new(headline: String, location: String, author: String, content: String) -> NewsArticle {
        NewsArticle {
            headline,
            location,
            author,
            content,
        }
    }
}

impl Summary for NewsArticle {}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Summary missing")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn summary(content: impl Summary) {
    println!("{}", content.summarize());
}

fn summary2<T: Summary>(content: T) {
    println!("{}", content.summarize());
}

fn main() {
    let tweet = Tweet {
        username: "bo".to_string(),
        content: "mah tweet".to_string(),
        reply: true,
        retweet: true,
    };

    println!("{}", tweet.summarize());

    let article = NewsArticle::new(
        "bo headline".to_string(),
        "bo locaiton".to_string(),
        "bo".to_string(),
        "mah content".to_string(),
    );

    println!("{}", article.summarize());

    summary(article);
    summary2(tweet);
}
