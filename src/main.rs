mod point;
mod sierpinski_triangel;

use crossterm::style::Colorize;

use sierpinski_triangel::Runopt;
use sierpinski_triangel::SierpinskiTriangel;

use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let iteration_count: u32 = 100000;
    let mut run_opt: Runopt = Runopt::PrintImage; 
    match args.len(){
        1 => (),
        2 => {
            match args[1].as_str() {
                "-f"|"--File" =>(),
                "-t"|"--Terminal" => run_opt=Runopt::PrintTerminal,
                "-h"|"--Help" => {
                        help(false); 
                        return
                },
                _ => {
                    help(true);
                    return
                },
            }
        },
        _ => {
            help(true);
            return
        },
    }
    let algo = SierpinskiTriangel::init(iteration_count);
    algo.run(&run_opt);    
}

fn help(on_error: bool){
    
    match on_error {
        true => println!("{}","Called With Invalid Arguments".red().on_dark_grey()),
        _ => println!("\n")
    }
    println!("Sirpinski Triangel");
    println!("USAGE:\n\t striangel.exe [OPTIONS]");
    println!("OPTIONS:");
    println!("\t -f --File       Print output as png image");
    println!("\t -t --Terminal   Print output in terrminal");
    println!("\t -h --Help       Print help Informations");

}
