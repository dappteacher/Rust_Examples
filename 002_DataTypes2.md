These are simple examples for each of the main data types in Rust:

### Scalar Types

1. **Integers**
   ```rust
   let signed: i32 = -10;
   let unsigned: u32 = 10;
   println!("Signed: {}, Unsigned: {}", signed, unsigned);
   ```

2. **Floating-Point Numbers**
   ```rust
   let float32: f32 = 3.14;
   let float64: f64 = 2.718281828459;
   println!("f32: {}, f64: {}", float32, float64);
   ```

3. **Boolean**
   ```rust
   let boolean: bool = true;
   println!("Boolean: {}", boolean);
   ```

4. **Character**
   ```rust
   let character: char = 'Z';
   println!("Character: {}", character);
   ```

### Compound Types

1. **Tuples**
   ```rust
   let tuple: (i32, f64, char) = (42, 3.14, 'a');
   let (x, y, z) = tuple;
   println!("Tuple: ({}, {}, {})", x, y, z);
   ```

2. **Arrays**
   ```rust
   let array: [i32; 3] = [1, 2, 3];
   println!("Array: {:?}", array);
   ```

### Additional Types

1. **Slices**
   ```rust
   let array: [i32; 3] = [1, 2, 3];
   let slice: &[i32] = &array[1..];
   println!("Slice: {:?}", slice);
   ```

2. **String and String Slices**
   ```rust
   let string: String = String::from("Hello, Rust!");
   let string_slice: &str = &string[0..5];
   println!("String: {}, String Slice: {}", string, string_slice);
   ```

3. **Vectors**
   ```rust
   let vec: Vec<i32> = vec![1, 2, 3];
   println!("Vector: {:?}", vec);
   ```

4. **Enums**
   ```rust
   enum IpAddr {
       V4(u8, u8, u8, u8),
       V6(String),
   }
   let home = IpAddr::V4(127, 0, 0, 1);
   let loopback = IpAddr::V6(String::from("::1"));
   println!("Home: {:?}, Loopback: {:?}", home, loopback);
   ```

5. **Structs**
   ```rust
   struct Point {
       x: f64,
       y: f64,
   }
   let point = Point { x: 1.0, y: 2.0 };
   println!("Point: ({}, {})", point.x, point.y);
   ```

6. **Option and Result**
   ```rust
   let some_number: Option<i32> = Some(5);
   let no_number: Option<i32> = None;
   println!("Some number: {:?}, No number: {:?}", some_number, no_number);

   let success: Result<i32, &str> = Ok(42);
   let failure: Result<i32, &str> = Err("Something went wrong");
   println!("Success: {:?}, Failure: {:?}", success, failure);
   ```

7. **Ranges**
   ```rust
   for i in 0..5 {
       println!("Range value: {}", i);
   }
   ```

### Special Types

1. **Unit Type**
   ```rust
   fn return_unit() {
       println!("This function returns nothing");
   }
   let unit: () = return_unit();
   println!("Unit: {:?}", unit);
   ```

2. **References**
   ```rust
   let x = 10;
   let y: &i32 = &x; // Immutable reference
   println!("x: {}, y: {}", x, y);

   let mut z = 20;
   let z_ref: &mut i32 = &mut z; // Mutable reference
   *z_ref += 10;
   println!("z: {}", z);
   ```

These examples cover the basic usage of each type in Rust. 
They demonstrate how to declare, initialize, and use various data types in simple contexts.
