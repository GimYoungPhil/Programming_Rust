fn main() {
  let v: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4];
  let a: [f64; 4] = [0.5, 0.6, 0.7, 0.8];

  let sa: &[f64] = &a;
  let sv: &[f64] = &s;

  println!("sa: {}", sa);
  println!("sv: {}", sv);
}
