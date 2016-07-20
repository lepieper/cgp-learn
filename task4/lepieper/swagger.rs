fn main() {
    let h = Swagger { save: "FeelsGoodMan" };
    println!("{}", h);
}

struct Swagger<T> {
    save: T,
}

use std::fmt::*;


impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.save)
    }
}
