mod point;
mod sierpinski_triangel;

use sierpinski_triangel::Runopt;
use sierpinski_triangel::SierpinskiTriangel;

fn main() {
    let algo = SierpinskiTriangel::init(100000);
    algo.run(&Runopt::PrintTerminal);
}
