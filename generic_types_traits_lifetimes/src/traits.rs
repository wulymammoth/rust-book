pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub reteweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

pub fn shared_behavior() {
    let headline = String::from("Some Headline");
    let location = String::from("San Francisco");
    let author = String::from("wulymammoth");
    let content = String::from("foobar");
    let article = NewsArticle {
        headline,
        location,
        author,
        content,
    };
    println!("news article summary: {}", article.summarize());

    let username = String::from("wulymammoth");
    let content = String::from("dis a tweet");
    let reply = false;
    let reteweet = false;
    let tweet = Tweet {
        username,
        content,
        reply,
        reteweet,
    };
    println!("tweet summary: {}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}
