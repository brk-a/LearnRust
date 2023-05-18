use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    //parse strings into floats
    let first_num: f32 = first.parse().unwrap();
    let second_num = second.parse::<f32>().unwrap(); // x: f32 and y.parse::<f32>() are equivalent
    
    let result = operate(operator, first_num, second_num);
    println!("{:?}", output(first_num, operator, second_num, result));
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32{
    match operator{
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*'|'x'|'X' => first_num * second_num,
        '/' => if second_num != 0.0 {
                first_num / second_num
            } else {
                panic!("ZeroDivisionError: cannot divide by 0")
            },
        _ => panic!("ValueError: Invalid operator")
    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String{
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}

//
//using if statements instead of match
//
// if operator == '+'{
//     return first_num + second_num;
// }
// if operator == '-'{
//     return first_num - second_num;
// }
// if operator == '*'{
//     return first_num * second_num;
// }
// if operator == '/'{
//     if second_num != 0{
//         return first_num / second_num;
//     }
//     return panic!("ZeroDivisionError: cannot divide by 0";
// }
// return "ValueError: Invalid operator";