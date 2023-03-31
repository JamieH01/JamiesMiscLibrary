# Macros


## watch!
A simple macro used for watching variables in the terminal. By default prints the name of the variable, but you can also include custom headers. I plan on making this much more involved in the future.

```
watch!(var_a, var_b, var_c);
watch!("Variable A":var_a);
```

## array2d!
Initializes an Array2d type. Read more on them in the 'Structs' section.
Takes a width, height, and default value + type. Can also create an Array2d from a regular vector with a semicolon.

```
//array2d!(width, height, type, default)
let table_a = array2d!(50, 50, u32, 15);
let table_b = array2d!(15, 10; vector);
```

## point!
Initializes a Point2/3 type, depending on how many parameters are provided. Read more on them in the 'Structs' section.
Takes an x, y, and optional z value (that must be a number type).

```
//point!(x, y, z)
let 2d_point = point!(12, 5);
let 3d_point = point!(25, 16, 8);
```

## vel!
Initializes a Velocity type, taking either a `!` or a `?` to signify radians or degrees. Note that this is only for the *initialized* value, as the internal `dir` variable is simply in radians. Read more on them in the 'Structs' section.

```
//vel!(dir, mag)
let rad_vel = vel!(? 0.3, 15); //radians
let deg_vel = vel!(! 25, 8); //degrees
```

## try_it!
A simple macro that shortens `try_into().unwrap()` to make code look a little nicer.

```
let int:u32 = 15;
let usize_int:usize = try_it!(int);
```

## keys!
Minimizes the boilerplate of matching held keys using the `device_query` crate. This macro will not work without it.

```
use device_query::{DeviceQuery, DeviceState, Keycode};

let device_state = DeviceState::new();

keys!(device_state, {
    Keycode::Escape => {return}, 
    _ => {break}
});
```

# Functions

## from_u8_rgb
Converts 3 u8 color channel values into a single u32 to be used in stuff like minifb.

```
let red = from_u8_rgb(255, 0, 0);
```

## distance
Gets the distance between 2 x and y values.

```
let distance = distance(15.0, 29.0, 243.0, 44.0);
```
*this is available in a more straightforward way on the point structs*

## dot_product_f64
Gets the dot product of two Vec<f64> of equal length.

```
let VectorA = vec![2.0; 10];
let VectorB = vec![3.0; 10];
let product = dot_product_f64(VectorA, VectorB);
```

## matrix_mul_f64
Multiplies two Array2d<f64> together.

```
let mul_tableA = array2d!(50, 30, f64, 2.0);
let mul_tableB = array2d!(10, 50, f64, 3.0);
let matrix = matrix_mul_f64(mul_tableA, mul_tableB);
```

# Structs

## Array2d
Probably the most useful thing here. An Array2d is a vector with width and height fields, allowing it to be indexed as both a 1 and 2 dimensional table. it also has an optional position lookup table to make converting from an index to a position faster, with the downside of an initial hitch when initializing. I generally recommend you set it on.

### new / new_from
Creates a new table from either a default value or existing vector. Dont worry about these, use the macro `array2d!`.

### nth
Indexes the table at a value. Effectively identical to `data[i]`. Note that the `data` field that holds the vector is public so you can change it from there if you'd prefer. It just felt better to include this for completeness.
```
let value = table.nth(15);
let value = table.data[15];
```

### pos
Indexes the table at a position.
```
let value = table.pos(3,4);
```

### set_nth
Sets the value at an index.
```
table.set_nth(20, value);
table.data[20] = value;
```

### set_pos
Sets the value at a position
```
table.set_pos(32, 12, value);
```

### nth_to_pos
Converts an index to its corresponding position.
```
let position:(usize, usize) = table.nth_to_pos(15);
```

### pos_to_nth
Converts a position to its corresponding index.
```
let index = table.pos_to_nth(13, 8);
```

### iter
An iterator over the vector so that you dont have to call it from the data field.
```
almost never use iterators so ¯\_(ツ)_/¯
```

### get_row
Returns a vector with every element from the row of a table.
```
let row = table.get_row(15);
```

### get_column
Returns a vector with every element from the column of a table. these are used mostly for the matrix_mul function.
```
let column = table.get_colum(10);
```

### pop_data
Frees and returns the vector from within a table.
```
let data = table.pop_data() //<- table is freed
```

### is_bounded
Checks if x and y values are within the dimensions of the table to avoid width under/overflows, by indexing values out of bounds
```
if table.is_bounded(15, 4) {todo()!}
```

### properties
These are the available read-only properties you can get from a table:
```
table.width()
table.height()
table.len()
```

## Point2/3
All of the methods are the same, just implemented for 2 or 3 dimensions.

### basic arithmetic
`add(), sub(), mul(),` and `div()` are available for basic arithmetic with other points. `distance()` is also there to make getting distances with points more straightforward.

### const arithmetic
`const_add(), const_sub(), const_mul()` and `const_div()` are available for applying a cosntant to every element of a point.
```
let p1 = point!(4, 5, 6);
let p2 = p1.const_add(1);
assert_eq!(p2, point!(5, 6, 7));
```

### as_velocity
Only available on Point2. Converts a cartesian coordinate to a vector coordinate (returning a `Velocity` type), assuming the tail is at the origin.
```
let vector = point.as_velocity();
```

## Velocity
A 2 dimensional vector with direction and magnitude.

### basic arithmetic
`add(), sub(), mul(),` and `div()` are available for basic arithmetic with other Velocities.

### as_point
Converts a vector coordinate to a cartesian coordinate (returning a `Point2` type).
```
let point = vector.as_point();
```

## WindowContainer
A holder for `minifb` windows to make managing them simpler. The internal `buffer` is the pixel buffer used by the window. You can modify this Array2d directly. 

### new
Creates a new window. Takes width, height, name, and background color.
```
let mut window = WindowContainer::new(1000, 1000, "Name", Color::Grey.value());
```

### update
updates the window with whatever data is in the buffer. Usually should be called at the end of a loop. (Note that this code has no way of terminating)
```
loop {
    window.update();
}
```



# Enums

## Color
A simple enum that holds some basic colors.
available colors:
```
Red    
Green  
Blue   
Yellow 
Cyan   
Magenta
Orange 
Blank  
White  
```

### value
Gets the u32 rgb value from the color it is called on.
```
let red = Color::Red.value();
```