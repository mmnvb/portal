struct Position{
    x: f64,
    y: f64,
    a: f64,
}

fn read_input() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Enter valid arguments!")
}


impl Position{
    fn new(i: u8) -> Position{
        println!("Enter x{}: ", i);
        let x:f64 = read_input();
        println!("Enter y{}: ", i);
        let y:f64 = read_input();
        println!("Enter angle {}: ", i);
        let a:f64 = read_input();

        Position { x, y, a}        
    }
}

struct LinearEquation{
    k:f64,
    b:f64,
}

impl LinearEquation{
    fn new(pos: Position) -> LinearEquation{
        let k = (pos.a + 90.0).to_radians().tan();
        LinearEquation {k, b: pos.y - k * pos.x}
    }

    fn display(&self){
        println!("Equation for it: y={}x+{}", &self.k, &self.b);
    }
}

fn calculate_intersection(e1: LinearEquation, e2: LinearEquation) -> Result<(f64, f64), String>{
    let d = -e1.k + e2.k;
    if d == 0.0 {
        return Err(String::from("The lines are parallel!"));
    }
    let d_x = e1.b - e2.b;
    let d_y = e1.k * (-e2.b) + e1.b * e2.k;

    Ok((d_x / d, d_y / d))
}

fn main(){
    println!("WELCOME TO PORTAL FINDER (Rust version)!\n");

    // get data to create a linear equation
    let first = LinearEquation::new(Position::new(1)); 
    first.display();
    let second = LinearEquation::new(Position::new(2)); 
    second.display();

    // show the results
    match calculate_intersection(first, second){
        Ok(intersection) => {
            use std::io::Write;
            use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

            let mut stdout = StandardStream::stdout(ColorChoice::AlwaysAnsi);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).expect("went wrong");
            writeln!(&mut stdout, "\nResult: {} ~ {}", intersection.0, intersection.1).expect("wen wrong");
        },
        Err(e) => eprintln!("Error occured: {}", e),
    }
    

    read_input();
}