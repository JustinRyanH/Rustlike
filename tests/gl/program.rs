use rspec::given;
use expect;

#[test]
fn main() {
    let example: Example = Default::default();
    rspec::run(&given("Rspec is setup", 0, |ctx| {
        ctx.it("then it will successfully run specs", |_| true);
    }));
}
