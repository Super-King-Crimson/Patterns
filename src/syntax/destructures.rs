pub fn explain() {
    let origin = Vector3::new();
    let Vector3 { x, y, z: hey_you_can_also_rename_these } = origin;
    println!("({x}, {y}, {hey_you_can_also_rename_these})");

    //.. to ignore the rest of the values in a struct destructure, _ to ignore in an irrefutable pattern
    if let Vector3 { x: 0, .. } = origin {
        println!("On Y axis");
    }

    if let Vector3 { y: 0, .. } = origin {
        println!("On X axis");
    }

    if let Vector3 { z: 0, .. } = origin {
        println!("On Z axis");
    }

    let (a1, b2, _, _, three, .., five) = ("c3", "d4".to_string(), 1, 2, 3, 4, 5);
    println!("{a1}, {b2}, {three}, {five}");

}

pub struct Vector3 {
    x: i32,
    y: i32,
    z: i32
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 {
            x: 0,
            y: 0,
            z: 0
        }
    }
}