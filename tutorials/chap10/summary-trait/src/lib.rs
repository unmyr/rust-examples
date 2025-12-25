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
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &(impl Summary + Display)) {
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                     hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tweet() {
        use crate::Summary;
        use crate::Tweet;

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        assert_eq!(
            tweet.summarize(),
            "horse_ebooks: of course, as you probably already know, people"
        );
    }

    #[test]
    fn test_news_article() {
        use crate::NewsArticle;
        use crate::Summary;

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        assert_eq!(
            article.summarize(),
            "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)"
        );
    }

    #[test]
    fn test_multiple_return_types() {
        use crate::returns_summarizable;
        let summary = returns_summarizable(true);
        assert_eq!(
            summary.summarize(),
            "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)"
        );

        let summary = returns_summarizable(false);
        assert_eq!(
            summary.summarize(),
            "horse_ebooks: of course, as you probably already know, people"
        );
    }
}
