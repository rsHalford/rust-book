fn main() {
    print_labelled_measurement(5, 'h');
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
