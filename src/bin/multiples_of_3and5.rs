const NUMBER1: i32 = 3;
const NUMBER2: i32 = 5;
const NUMBER3: i32 = NUMBER1 * NUMBER2;
const MIN: i32 = 0;
const MAX: i32 = 1000 - 1;

fn main() {
  let number1_sum =
    NUMBER1 * ((MIN / NUMBER1 + 1) + MAX / NUMBER1) * (MAX / NUMBER1 - (MIN / NUMBER1 + 1) + 1) / 2;
  let number2_sum =
    NUMBER2 * ((MIN / NUMBER2 + 1) + MAX / NUMBER2) * (MAX / NUMBER2 - (MIN / NUMBER2 + 1) + 1) / 2;
  let number3_sum =
    NUMBER3 * ((MIN / NUMBER3 + 1) + MAX / NUMBER3) * (MAX / NUMBER3 - (MIN / NUMBER3 + 1) + 1) / 2;
  println!("{}", (number1_sum + number2_sum - number3_sum));
}
