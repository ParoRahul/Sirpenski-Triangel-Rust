use std::io::{stdout, Write};

use super::point::Point;

use crossterm::{QueueableCommand, Result};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand,
};

use image::{ImageBuffer, Luma};

use rand::{thread_rng, Rng};

const IMAGE_HEIGHT: u32 = 600;
const IMAGE_WIDTH: u32 = 800;

pub enum Runopt {
    PrintImage,
    PrintTerminal,
}

#[warn(dead_code)]
pub struct SierpinskiTriangel {
    iteration_count: u32,
}

impl SierpinskiTriangel {
    pub fn init(iteration_count: u32) -> SierpinskiTriangel {
        SierpinskiTriangel { iteration_count }
    }

    fn corner_points(&self, runopt: &Runopt) -> [Point; 3] {
        match runopt {
            Runopt::PrintImage => [
                Point::from(0, IMAGE_HEIGHT),
                Point::from(IMAGE_WIDTH / 2, 0),
                Point::from(IMAGE_WIDTH, IMAGE_HEIGHT),
            ],
            Runopt::PrintTerminal => {
                let (cols, rows) = terminal::size().unwrap();
                let rows = (f64::from(rows) * 0.95) as u32;
                [
                    Point::from(1, rows),
                    Point::from((cols / 2) as u32, 1),
                    Point::from(cols as u32, rows),
                ]
            }
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

    pub fn run(&self, runopt: &Runopt) {
        let mut init_point: Point = Point::from(0, 0);
        let mut counter = self.iteration_count.clone();
        let mut rng = thread_rng();
        let mut result_points: Vec<Point> = Vec::new();
        let corner_point: [Point; 3] = self.corner_points(runopt);
        while counter > 0 {
            let index = rng.gen_range(0..3);
            let rand_point = corner_point[index];
            init_point = Point::from(
                (init_point.x() + rand_point.x()) / 2,
                (init_point.y() + rand_point.y()) / 2,
            );
            result_points.push(init_point);
            counter = counter - 1;
        }
        match runopt {
            Runopt::PrintImage => self.run_image(&mut result_points),
            Runopt::PrintTerminal => self.run_terminal(&mut result_points),
        }
    }

    fn run_image(&self, list_of_point: &mut Vec<Point>) {
        let mut image_result = self.create_image();
        let pixel = image_result[(0, 0)];
        while let Some(point) = list_of_point.pop() {
            image_result.put_pixel(point.x(), point.y(), pixel);
        }
        let _ = image_result.save("Sierpinski_Triangel.png").unwrap();
    }

    fn run_terminal(&self, list_of_point: &mut Vec<Point>) {
        let mut stdout = stdout();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        let (_, rows) = terminal::size().unwrap();
        let rows = (f64::from(rows) * 0.95) as u16;
        while let Some(point) = list_of_point.pop() {
            execute!(
                stdout,
                cursor::MoveTo(point.x() as u16, point.y() as u16),
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black),
                Print("*".to_string()),
                ResetColor
            )
            .unwrap()
        }
        stdout
            .queue(cursor::MoveTo(10, rows + 2))
            .unwrap()
            .flush()
            .unwrap();
    }
}
