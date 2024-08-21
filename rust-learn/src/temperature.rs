pub fn convert_celsius_to_fahrenheit(temperature:f64) -> String {
    let value :f64 =  (temperature * 1.8) + 32.0;
    return format!("Temperature in Fahrenheit is : {}°F" , value);
}