fn main() {
    println!("Hello, world!");
}

#[test]
fn it_doesnt_work() {
    use expect_test::expect;
    use expect_test::Expect;

    fn t(expect: &Expect) {
        expect.assert_eq("\n")
    }
    t(&expect![[r#"

    "#]]);
}
