use std::{str, panic, fmt::format};

fn main() {
    let set_1_degree:f64 = 70.0;
    let set_1_measurement = "F";
    let set_1_output = convert_temperature(set_1_degree, set_1_measurement);
    let set_1_new_measurement = change_measurement(set_1_measurement); 

    let set_2_degree = 21.1111;
    let set_2_measurement = "C";
    let set_2_output = convert_temperature(set_2_degree, set_2_measurement);
    let set_2_new_measurement = change_measurement(set_2_measurement);

    let set_2_string = format!("{set_2_output}*{set_2_new_measurement}");

    println!("Set 1 conversion of {set_1_degree}*{set_1_measurement}: {set_1_output}*{set_1_new_measurement}");
    println!();

    println!("Set 2 conversion of {set_2_degree}*{set_2_measurement}: {set_2_output}*{set_2_new_measurement}");
    println!();

    println!("Testing with creating string in set 2 variable: {set_2_string}");
    println!("=======");
    println!("String func output:");
    let full_converted_set_1 = full_conversion(set_1_degree, set_1_measurement);
    let full_converted_set_2 = full_conversion(set_2_degree, set_2_measurement);
    println!("{full_converted_set_1}");
    println!("{full_converted_set_2}");
/* Notes:
# RFER 06
    - Using function to return String:
        - Link:
            - https://users.rust-lang.org/t/how-to-return-str/67385/3

    - Creating "String"
        - Link:
            - https://stackoverflow.com/a/68371061
*/
} 

fn convert_temperature(degree:f64, measurement:&str) -> f64 {

    match measurement {
        "F" => {
            // Convert from F to C
            ({degree} - 32.0) * 5.0 / 9.0
            
        },
        "C" => {
            // Convert from C to F
            ({degree} * (9.0/5.0)) + 32.0
        },
        _ => panic!()
    }
}

fn change_measurement(measurement:&str) -> String {
    match measurement {
        "F" => "C".to_string(),
        "C" => "F".to_string(),
        _ => panic!(),
    }
}

fn full_conversion(degree:f64, measurement:&str) -> String {

    match measurement {
        "F" => {
            // Convert from F to C
            let outcome = ({degree} - 32.0) * 5.0 / 9.0;
            format!("{outcome}*C")
            
        },
        "C" => {
            // Convert from C to F
            let outcome = ({degree} * (9.0/5.0)) + 32.0;
            format!("{outcome}*F")
        },
        _ => panic!()
    }
}