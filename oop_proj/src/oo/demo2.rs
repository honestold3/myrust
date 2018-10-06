pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        println!("============new============");
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        println!("============add_text============");
        self.content.push_str(text);
    }

//    pub fn content(&self) -> &str {
//        ""
//    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        println!("request_review for Draft");
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        println!("approve for Draft");
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        println!("request_review for PendingReview");
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub fn kankan(){
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("1.after add_text-----{}", post.content());

    post.approve();

    post.request_review();
    println!("2.after request_review----{}",post.content());

    post.request_review();
    println!("3.after request_review----{}",post.content());

    post.approve();
    println!("4.approve========={}",post.content());
}