pub trait Summarizable {
    fn summary(&self) -> String{
        String::from("Read more....")
    }

    fn author_summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, byee {} ({})", self.headline, self.author, self.location)
    }

    fn author_summary(&self) -> String {
        format!("author@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author_summary(&self) -> String {
        format!("username::@{}", self.username)
    }
}

//------------------------------------------------------

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_perdipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitation is {}%.",
                self.high_temp, self.low_temp,
                self.chance_of_perdipitation)
    }

    fn author_summary(&self) -> String {
        format!("high_temp::@{}", self.high_temp)
    }
}

//--------------------------------------------------------

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news::: {} {}",item.summary(),item.author_summary());
}

pub fn notify1<T>(item: T)
    where T: Summarizable {
    println!("Breaking news::: {} {}",item.summary(),item.author_summary());
}


//------------------------------------------------------
pub fn kankan(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {} - {}", tweet.summary(),tweet.author_summary());

    let my_article = NewsArticle {
        headline: String::from("jjjj"),
        location: String::from("aaaa"),
        author: String::from("eeeeee"),
        content: String::from("ttttt"),
    };
    println!("1 new article: {} - {}", my_article.summary(),my_article.author_summary());

    let weather = WeatherForecast{
         high_temp: 64 as f64,
         low_temp: 64 as f64,
        chance_of_perdipitation: 64 as f64,
    };
    println!("weather: {}-{}",weather.summary(),weather.author_summary());

    notify(tweet);
    notify(my_article);
    notify(weather);
}
//------------------------------------------------------

