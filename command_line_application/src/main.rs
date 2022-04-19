use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut weight = input.trim().parse().unwrap();
    let  mars_weight  = calculate_weight_on_mars(weight);
    println!(" Weight :{}", mars_weight);
    

}
fn calculate_weight_on_mars(_weight: f32)->f32{
    (_weight/9.81)*3.711
}