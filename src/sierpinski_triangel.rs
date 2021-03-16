extern crate image;
extern crate rand;

use super::point::Point;

use image::{ImageBuffer, Luma};
use rand::{thread_rng, Rng};

const IMAGE_HEIGHT: u32 = 600;
const IMAGE_WIDTH: u32 = 800;

pub enum Runopt {
    PrintImage,
    PrintTerminal,
}

pub struct SierpinskiTriangel {
    iteration_count: u32,
    corner_point: [Point; 3],
}

impl SierpinskiTriangel {
    pub fn init(iteration_count: u32) -> SierpinskiTriangel {
        SierpinskiTriangel {
            iteration_count,
            corner_point: [
                Point::from(0, IMAGE_HEIGHT),
                Point::from(IMAGE_WIDTH / 2, 0),
                Point::from(IMAGE_WIDTH, IMAGE_HEIGHT),
            ],
        }
    }

    fn create_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
            if x == 0 && y == 0 {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        })
    }

    pub fn run(&self, runopt: Runopt) {
        match runopt {
            Runopt::PrintImage => self.run_image(),
            Runopt::PrintTerminal => self.run_image(),
        }
    }

    fn run_image(&self) {
        let mut init_point = Point::from(IMAGE_WIDTH / 2, 0);
        let mut counter = self.iteration_count;
        let mut rng = thread_rng();
        let mut image_result = self.create_image();
        // let pixel = image_result[(0, 0)];
        let pixel = image::Luma([0u8]);
        while counter > 0 {
            let index = rng.gen_range(0..3);
            let rand_point = self.corner_point[index];
            init_point = Point::from(
                (init_point.x() + rand_point.x()) / 2,
                (init_point.y() + rand_point.y()) / 2,
            );
            image_result.put_pixel(init_point.x(), init_point.y(), pixel);
            counter = counter - 1;
        }
        let _ = image_result.save("SierpinskiTriangel.png").unwrap();
    }

    fn run_terminal(&self) {
        print!("This is not implemented Yet")
    }
}
