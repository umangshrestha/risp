
fn main() {

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BoolLiteral(bool);

    let a = BoolLiteral(true);
    println!("{:?}", a==BoolLiteral(false));
}
