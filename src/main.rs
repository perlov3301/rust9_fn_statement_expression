 fn main() {
    println!("rust9_functions_expressions");
    ftest_one("test");
    ftest_one("call");
    fadd_one(1.0,2.0);
// on right side allowed expression only of the same type,
// no statement neither function
    let x= 9.+1.0;
    let y= {
        let x = 3.0;
// expression without semicolon
        x+1.0
     };
     println!{"y is {} ", y};
     fadd_one(x,y);
  // fadd_two=expression (has returned value)
     let mut a1= fadd_two(2.0,3.0);
     println!("returned value of fadd_two is {}", a1);
     a1 = f_expression();
     println!("returned value of f_expression is {}", a1);
}
  fn ftest_one(a:&str ) {
    println!("{} of outside function",a);
}
  fn fadd_one(a1:f32, a2:f32) {
    println!("function fadd_one");
    println!("sum of {} and {} is  {}", a1, a2, a1+a2);
}
// "->" is return operator in rust for defining type
  fn fadd_two(a1:f32, a2:f32) -> f32 {
    println!("fadd_two function");
    println!("sum of {} and {} is {}  ",a1,a2,a1+a2);
// returned value
    a1+a2
  }
  fn f_expression() -> f32 {
    10.0
  }

