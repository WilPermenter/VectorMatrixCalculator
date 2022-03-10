use std::io;

struct Array{
    x: usize,
    y: usize,
    data: String,
}

fn main() {
    println!("8==== Long Shlong Vector Matrix Calculator  ====D");
    let mut user_input = String::new();

    println!("Input your first array in the format 'x, y, x1,x2...' Denote angles with * ex 100∠0 = '100*0' ");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //Example Input 2,2,1,1,1,100*0

    let array_one = build_array(&user_input);

    user_input = "".to_owned();

    println!("Input your second array in the format 'x, y, x1,x2...' Denote angles with * ex 100∠0 = '100*0' ");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //Example Input 2,2,1,1,1,100*0

    let array_two = build_array(&user_input);
    //Example data : x = 2 , y = 2 , data = 1,1,1,100*0,

    println!("Answers: {}",solve_da_hoe(array_one,array_two));
    
    //let answer = solveDaHoe(array_one,array_two);
}

fn build_array(user_input: &str) -> Array{
    let temp_vec: Vec<&str> = user_input.split(",").collect();

    let mut data = String::from("");

    for num in 2..temp_vec.len(){
        data.push_str(temp_vec[num].trim());
        data.push_str(",");
    }

    //Yes I know I did this entire thing with x / y backwards but Im to lazy to fix it so yea... It gets fliped here
    let arr = Array {x : temp_vec[1].parse().unwrap(), y : temp_vec[0].parse().unwrap() , data : data };

    arr 
}

fn solve_da_hoe(arr_one: Array, arr_two: Array) -> String{
    //Get Data Ready
    let vec_one: Vec<&str> = arr_one.data.split(",").collect();
    let vec_two: Vec<&str> = arr_two.data.split(",").collect();
    let mut multi_data: String ="".to_owned();
    let mut added_data: String ="".to_owned();

    //multi x = b.x
    //multi y = a.y

    for b_col in 0..arr_two.x{
        for a_row in 0..arr_one.y{
            for num in 0..arr_one.x{
                multi_data.push_str(multiply_vectors(vec_one[(a_row * arr_one.x) + num],vec_two[(num * arr_two.x) + b_col]).as_str());
            }
        }
    }
    println!("{}",multi_data);
    let multi_data_vec: Vec<&str> = multi_data.split(",").collect();

    let mut output: String;
    let mut temp: &str;
    
    for row in 0 ..arr_two.x {
        for col in 0..arr_one.y {
            temp = multi_data_vec[(row * arr_one.y ) +(col * arr_one.y)];
            for com in 1..arr_one.x{
                output = add_vectors(temp,multi_data_vec[(row * arr_one.y ) + (col * arr_one.y) + com]);
                temp = output.as_str();
            }
            added_data.push_str(temp);
        }
    }
    added_data
    
}

fn to_int(num: &str) -> (f32,f32){
    let mut tuple = (0.0_f32,(-1.0_f32).sqrt());
    let vec: Vec<&str> = num.split("*").collect();
    println!("{}",vec[0]);
    tuple.0 = vec[0].parse::<f32>().unwrap();
    if vec.len() == 2{
        if vec[1] != "NaN"{
            tuple.1 = vec[1].replace(",","").parse::<f32>().unwrap();
        }
    }

    tuple
}

fn multiply_vectors(num_one: &str, num_two: &str) -> String{
    let t_one = to_int(num_one);
    let t_two = to_int(num_two);
    
    if t_one.1.is_nan() && t_two.1.is_nan() {
        return format!("{},",(t_one.0)*(t_two.0));
    }else if !t_one.1.is_nan() && t_two.1.is_nan(){
        return format!("{}*{},",(t_one.0)*(t_two.0),t_one.1);
         
    }else if t_one.1.is_nan() && !t_two.1.is_nan(){
        return format!("{}*{},",(t_one.0)*(t_two.0),t_two.1);
    }
    return format!("{}*{},",(t_one.0)*(t_two.0),(t_one.1 + t_two.1));
}

fn add_vectors(num_one: &str, num_two: &str) -> String{
    let t_one = to_int(num_one);
    let t_two = to_int(num_two);

    if t_one.1.is_nan() || t_two.1.is_nan() {
        panic!("Cannot Add Scalear & Vector");
    }

    //If Vector
    let t_one_r = t_one.1.to_radians();
    let t_two_r = t_two.1.to_radians();

    //Issue Exists Here?
    let x_one = t_one.0 * t_one_r.cos();
    let x_two = t_two.0 * t_two_r.cos();
    let y_one = t_one.0 * t_one_r.sin();
    let y_two = t_two.0 * t_two_r.sin();

    let x = x_one + x_two;
    let y = y_one + y_two;
    

    let r = (x*x + y*y).sqrt();
    let mut th = (y/x).atan().to_degrees();

    if x < 0.0_f32{
        th += 180.0;
    }
    if th < 0.0 {
        th += 360.0;
    }
    if th > 360.0{
        th -= 360.0;
    }

    return format!("{}*{},",r,th);

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let one = Array {x: 3, y:3,data:"1,1,1,1,1*240,1*120,1,1*120,1*240".to_string()};
        let two = Array {x: 1, y:3,data:"50*80,100*0,50*90".to_string()};

        assert_eq!("147.17513*42.399918,105.11607*216.38937,110.86068*88.974724,", solve_da_hoe(one,two));
    }
    #[test]
    fn test_two() {
        let one = Array {x: 3, y:3,data:"1,1,1,1,1*240,1*120,1,1*120,1*240".to_string()};
        let two = Array {x: 1, y:3,data:"60*80,120*0,50*90".to_string()};

        assert_eq!("170.02759*39.910686,116.20672*216.93806,138.15428*92.605286,", solve_da_hoe(one,two));
    }
    #[test]
    fn test_three() {
        let one = Array {x: 3, y:2,data:"-80*290,99.55*777,80*0,1,1*240,1*120".to_string()};
        let two = Array {x: 1, y:3,data:"50*36.87,50*10,50*177".to_string()};

        assert_eq!("---", solve_da_hoe(one,two));
    }
}