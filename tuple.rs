struct Color(u8, u8, u8);
struct Kilometers(i32);

fn main() {
  // Creating an instance
  let black = Color(0, 0, 0);

  // Destructure the instance using a `let` binding, this will not destruct black instance
  let Color(r, g, b) = black;
  println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);

  // Newtype pattern
  let distance = Kilometers(20);
  // Destructure the instance using a `let` binding
  let Kilometers(distance_in_km) = distance;
  println!("The distance: {} km", distance_in_km); //The distance: 20 km
}