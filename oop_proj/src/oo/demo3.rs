pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }

    pub fn content(&self) -> &str {
        ""
    }

}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn content(&self) -> &str {
        ""
    }
}

pub fn kankan(){
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("1.after add_text-----{}", post.content());

    let post = post.request_review();
    println!("2.after request_review----{}",post.content());

    let post = post.approve();
    println!("3.approve========={}",post.content());

    assert_eq!("I ate a salad for lunch today", post.content());


}
