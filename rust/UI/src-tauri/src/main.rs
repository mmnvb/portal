// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn main_backend(x1: f64, y1: f64, a1: f64, x2: f64, y2: f64, a2: f64) -> String {
        // get data to create a linear equation
        let first = LinearEquation::new(Position::new(x1, y1, a1)); 
        let second = LinearEquation::new(Position::new(x2, y2, a2)); 

        // show the results
        match calculate_intersection(first, second){
            Ok(intersection) => {
                return format!("{} ~ {}", intersection.0, intersection.1);
            },
            Err(e) => return format!("Error occured: {}", e),
        }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![main_backend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


struct Position{
    x: f64,
    y: f64,
    a: f64,
}


impl Position{
    fn new(x: f64, y: f64, a:f64) -> Position{
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