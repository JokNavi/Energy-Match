use crate::data::cube::Cube;

pub fn print_cube(cube: &Cube, vertical_size: usize) {
    if vertical_size <= 2 {
        println!("{}", cube.side.display_value)
    }
    let vertical_size = vertical_size - 2;
    let horizontal_size = vertical_size * 3;

    println!("|{0:-<horizontal_size$}|", "");
    for i in 0..vertical_size {
        if i == vertical_size / 2 {
            println!("|{0:^horizontal_size$}|", cube.side.display_value);
        } else {
            println!("|{0:^horizontal_size$}|", "");
        }
    }
    println!("|{0:-<horizontal_size$}|", "");
}
