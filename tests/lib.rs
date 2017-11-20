extern crate rustlike;
extern crate rspec;
extern crate expect;

use rspec::given;

#[test]
fn main() {
    rspec::run(&given("an basic rustlike", 0, |ctx| {
        ctx.it("then it will successfully run specs", |__| {
            true
        });
    }));
}
