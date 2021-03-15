mod point;
mod sierpinski_triangel;

use sierpinski_triangel::SierpinskiTriangel;

fn main() {
    //let img = image::open("D:/RUST/tuitorial/triangel/image/download.png").unwrap();

    // println!("dimensions {:?}", img.dimensions());
    let algo = SierpinskiTriangel::init(100000);

    algo.run();
}
