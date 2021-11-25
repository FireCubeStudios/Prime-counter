mod unit_tests;

fn main() 
{
  unit_tests::execute_unit_tests();
}

fn parse(input: i32) -> i32 {
  let mut val = 0;
  let mut is_loop = true;
  let mut index:i32 = 0;
  if input == 1 || input < 0
  {
      return val;
  }
  else if input > 2
  {
      val += 1;
  }
  while is_loop {    
    if input - index == 1
    {
      is_loop = false;
    }
    else if is_prime(input - index) == true
    {
      val += 1;
    }
    index += 1;
  }
  return val;
}

fn is_prime(input: i32) -> bool {
  let mut is_prime = true;
  let mut is_loop = true;
  let mut index:i32 = 2;
    while is_loop {    
      if index == input / 2 
      {
       is_loop = false;
      }
      else if input % index == 0
      {
        is_prime = false;
        is_loop = false;
      }
      index += 1;
    }
  return is_prime;
}
