use std::fmt::*;

fn main() {
    let h = Swagger { save: "FeelsGoodMan" };
    println!("{}", h);
}

struct Swagger<T> {
    save: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.save)
    }
}
