
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    // pub fn add_text(&mut self, text: &str) {
    //     self.content.push_str(text);
    // }

    /// 只允许 Draft 状态时编辑文章的方案实现
    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(&mut self.content, text);
    }

    /// 此时，该方法委托调用 State 的 content 方法
    pub fn content(&self) ->&str {
        self.state.as_ref().unwrap().content(&self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) ->Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn add_text<'a>(&self, post: &'a mut String, text: &str) {

    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) ->Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, post_content: &'a mut String, text: &str) {
        post_content.push_str(text);
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn demo1_for_post() {
    let mut post = Post::new();
    post.add_text("Draft 时更新内容 this is a demo post...");
    println!("{:#?}", &post.content());
    post.state = Some(post.state.unwrap().request_review());
    post.add_text("PendingReview 后更新内容 this is a demo post...");
    println!("{:#?}", &post.content());
    post.state = Some(post.state.unwrap().approve());
    post.add_text("Published 后更新内容 this is a demo post...");
    // 最后一次打印 Draft 时更新内容 this is a demo post...
    // PendingReview 或 Published 状态时更新内容是无效的。符合预期
    println!("{:#?}", &post.content());
}

fn main() {
    demo1_for_post();
}



