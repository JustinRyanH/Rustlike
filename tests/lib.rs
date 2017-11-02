extern crate rspec;
extern crate expect;

use rspec::given;

#[test]
fn main() {
    rspec::run(&given("Rspec is setup", 0, |ctx| {
        ctx.it("then it will successfully run specs", |_| true);
    }));
}
