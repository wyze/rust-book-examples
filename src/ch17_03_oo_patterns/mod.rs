use blog::{v1, v2};

fn process_pending(mut post: v2::PendingReviewPost) -> v2::Post {
    loop {
        post = match post.approve() {
            v2::ApproveResult::Approved(p) => return p,
            v2::ApproveResult::Pending(p) => p,
        }
    }
}

pub fn run() {
    let mut post = v1::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Can only add text while the post is in Draft state
    post.add_text(".");

    // Can now reject a post, which reverts state back to Draft
    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    // Post requires 2 approvals before state is changed to Published
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = v2::Post::new();

    post.add_text("I ate a salad for lunch today");

    // Below results in a compiler error
    // assert_eq!("", post.content());

    let post = post.request_review();
    let post = post.approve();
    let post = match post {
        v2::ApproveResult::Pending(p) => p.reject(),
        _ => panic!("We shouldn't reach this arm."),
    };
    let post = post.request_review();
    let post = process_pending(post);

    assert_eq!("I ate a salad for lunch today", post.content());
}

mod blog {
    pub mod v1 {
        pub struct Post {
            content: String,
            state: Option<Box<dyn State>>,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    content: String::new(),
                    state: Some(Box::new(Draft {})),
                }
            }

            pub fn add_text(&mut self, text: &str) {
                if self.state.as_ref().unwrap().can_add_text() {
                    self.content.push_str(text);
                }
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }

            pub fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(&self)
            }

            pub fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
        }

        trait State {
            fn approve(self: Box<Self>) -> Box<dyn State>;

            fn can_add_text(&self) -> bool {
                false
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }

            fn reject(self: Box<Self>) -> Box<dyn State>;

            fn request_review(self: Box<Self>) -> Box<dyn State>;
        }

        struct Draft {}

        impl State for Draft {
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn can_add_text(&self) -> bool {
                true
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview { approved: false })
            }
        }

        struct PendingReview {
            approved: bool,
        }

        impl State for PendingReview {
            fn approve(self: Box<Self>) -> Box<dyn State> {
                if self.approved {
                    Box::new(Published {})
                } else {
                    Box::new(PendingReview { approved: true })
                }
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }

            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct Published {}

        impl State for Published {
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }

            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
    }

    pub mod v2 {
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        pub struct PendingReviewPost {
            approved: bool,
            content: String,
        }

        pub enum ApproveResult {
            Pending(PendingReviewPost),
            Approved(Post),
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
                    approved: false,
                    content: self.content,
                }
            }
        }

        impl PendingReviewPost {
            pub fn approve(self) -> ApproveResult {
                if self.approved {
                    ApproveResult::Approved(Post {
                        content: self.content,
                    })
                } else {
                    ApproveResult::Pending(Self {
                        approved: true,
                        content: self.content,
                    })
                }
            }

            pub fn reject(self) -> DraftPost {
                DraftPost {
                    content: self.content,
                }
            }
        }
    }
}
