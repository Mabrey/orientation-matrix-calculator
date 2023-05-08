// To properly rotate the frame, we need to know what axes are being rotated, and by how much.

// Get input for each of the Axes

// Determine the order of the Axes rotation

use core::fmt;
use std::io;
// use ndarray::arr3;

enum UserMode {
    Easy,
    Normal,
}

#[derive(Debug,PartialEq)]
enum RotationAxes {
    X,
    Y,
    Z,
}

impl fmt::Display for RotationAxes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationAxes::X => write!(f, "X"),
            RotationAxes::Y => write!(f, "Y"),
            RotationAxes::Z => write!(f, "Z"),
        }
     }
}

enum RotationOrder {
    XYZ,
    XZY,
    YZX,
    YXZ,
    ZXY,
    ZYX,
}

fn main() {
    
    match get_mode_from_user() {
        UserMode::Easy => run_easy_mode(),
        UserMode::Normal => run_normal_mode(),
    };

    //Get rotation around X
    //Get rotation around Y
    //Get rotation around Z

    //Determine rotation order
    //XYZ
    //XZY
    //YZX
    //YXZ
    //ZXY
    //ZYX


}

fn run_easy_mode() {
    println!("Easy Mode Selected");
    let rotation_list: Vec<RotationAxes> = vec![RotationAxes::X, RotationAxes::Y, RotationAxes::Z];
    
    // Ask for first rotation Axis
    println!("Enter first axis of rotation.");
    let (rotation_list, first_axis) = get_axis_of_rotation_from_user(rotation_list);

    // Ask for first rotation in radians
    println!("Enter rotation about {first_axis} (in radians): ");

    // Ask for second rotation Axis
    println!("Enter second axis of rotation.");
    let (rotation_list, second_axis) = get_axis_of_rotation_from_user(rotation_list);

    // Ask for second rotation in radians
    println!("Enter rotation about {second_axis} (in radians): ");

    let third_axis = &rotation_list[0];

    // Ask for second rotation in radians
    println!("Enter rotation about {third_axis} (in radians): ");
    



    return;
}

fn run_normal_mode() {
    println!("Normal Mode Selected");
    return;
}

fn get_mode_from_user() -> UserMode {
    println!("Are you familiar with Matrix Math? (y/n)");
    let mut user_input: String = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_n) => {}
        Err(error) => println!("error: {error}"),
    }
    

    match user_input.trim() {
        "y" | "yes" | "Y" | "Yes" => return UserMode::Normal,
        "n" | "no"  | "N" | "No"  => return UserMode::Easy,
        _ => {
            println!("Invalid Input: {}", user_input.trim());
            println!("Try Again...");
            return get_mode_from_user();
        }
    };
}

fn get_axis_of_rotation_from_user(mut rotation_list: Vec<RotationAxes>) -> (Vec<RotationAxes>, RotationAxes) {
    println!("Options: {:?}", rotation_list);
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("invalid string");

    let axis = match user_input.trim() {
        "X" | "x" => RotationAxes::X,
        "Y" | "y" => RotationAxes::Y,
        "Z" | "z" => RotationAxes::Z,
        _ => {
            println!("Invalid String, Try again...");
            return get_axis_of_rotation_from_user(rotation_list);
        }
    };

    let found_rotation: Option<usize> = rotation_list.iter().position(|a| std::mem::discriminant(a) == std::mem::discriminant(&axis));
    if found_rotation.is_some() {
        println!("Selection {axis} found at index {}", found_rotation.unwrap());
        rotation_list.remove(found_rotation.unwrap());
        return (rotation_list, axis);
    } else {
        println!("Selection not valid, try again...");
        return get_axis_of_rotation_from_user(rotation_list);
    }

}
