extern crate rustlike;
extern crate rspec;
extern crate expect;

use std::sync::Arc;
use rspec::given;

use rustlike::{context};

#[derive(Clone, Debug)]
struct Example {
    builder: context::ContextBuilder,
}

impl Default for Example {
    fn default() -> Example {
        Example {
            builder: Default::default(),
            ctx: None,
        }
    }
}

impl Example {
    pub fn build(&mut self) {
        self.ctx = Some(self.builder.build().unwrap())
    }
}

#[test]
fn main() {
    rspec::run(&given("an basic rustlike", 0, |ctx| {
        ctx.it("then it will successfully run specs", |_| true);
    }));
}
