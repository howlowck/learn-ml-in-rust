extern crate clap;
use clap::{Arg, App};
mod simple_forward;
mod simple_learn;
mod colors;

fn run_project(name: &str, func: fn()) {
    println!("\n================== Running: {} ==================\n\n", name);
    func();
    println!("\n================== Completed: {} ==================\n\n", name);
}

fn main() {
    let matches = App::new("Hao's Learning ML in Rust")
                          .version("1.0")
                          .author("Hao Luo")
                          .about("Learning ML with Rust")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to run")
                               .required(true)
                               .index(1))
                          .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let name = matches.value_of("INPUT").unwrap();
    

    match name {
        "simple_forward" => run_project(&name, simple_forward::multi_to_multi_no_hidden_layers), // simple_forward::multi_to_one(),
        "simple_learn" => run_project(&name, simple_learn::simplest_learn),
        // Add a new project here!
        _ => println!("\n  sorry there is no project called {}", name)
    };
}