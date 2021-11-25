pub fn execute_unit_tests() 
{
    unit_test1();
    unit_test2();
    unit_test3();
    unit_test4();
    unit_test5();
    unit_test6();
    unit_test7();
    unit_test8();
    unit_test9();
}
fn unit_test1()
{
  let test = 20;
  println!("{} has {} prime numbers (8)", test, super::parse(test));
}
fn unit_test2()
{
  let test = 30;
  println!("{} has {} prime numbers (10)", test, super::parse(test));
}
fn unit_test3()
{
  let test = 100;
  println!("{} has {} prime numbers (25)", test, super::parse(test));
}
fn unit_test4()
{
  let test = 200;
  println!("{} has {} prime numbers (46)", test, super::parse(test));
}
fn unit_test5()
{
  let test = 1000;
  println!("{} has {} prime numbers (168)", test, super::parse(test));
}
fn unit_test6()
{
  let test = -5;
  println!("{} has {} prime numbers (0)", test, super::parse(test));
}
fn unit_test7()
{
  let test = 66;
  println!("{} has {} prime numbers (18)", test, super::parse(test));
}
fn unit_test8()
{
  let test = 133;
  println!("{} has {} prime numbers (32)", test, super::parse(test));
}
fn unit_test9()
{
  let test = 99;
  println!("{} has {} prime numbers (25)", test, super::parse(test));
}