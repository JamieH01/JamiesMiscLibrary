I generally find it fun to make my own structures and functions that fit my specific needs rather than find a crate that solves my problems in ways that actually make sense. This crate is meant to be my own library of macros, functions, and structs that ive found useful across multiple projects.

Feel free to use it, but i advise you dont, as most of the things here are not super well made, feature-complete, or polished for other users. Hopefully ill end up publishing more actually good tools, as this project will likely be the beta-breeding ground of those.

This will also likely be VERY unstable, as i dont expect anyone else to use this and wont account for much outside of my own projects, hence this being a dev-dep on githup instead of crates.io.

if you want need examples check out the test file for some simple ones.

Feel free to copy some of my code if you find it useful!


# Docs
(so i dont forget)

## Macros


### watch!
A simple macro used for watching variables in the terminal. By default prints the name of the variable, but you can also include custom headers. I plan on making this much more involved in the future.

```
watch!(var_a, var_b, var_c);
watch!("Variable A":var_a);
```

### array2d!
Initializes an Array2d type. Read more on them in the 'Structs' section.
Takes a width, height, and default value + type, along with an optional bool for whether a lookup-table should be written (generally should). Can also create an Array2d from a regular vector with a semicolon.

```
let table_a = array2d!(50, 50, u32, 15);
let table_b = array2d!(15, 10; vector ? true);
```

### point!
Initializes a Point2/3 type, depending on how many parameters are provided. Read more on them in the 'Structs' section.
Takes an x, y, and optional z value (that must be a number type).

```
let 2d_point = point!(12, 5);
let 3d_point = point!(25, 16, 8);
```

## Functions

### from_u8_rgb
Converts 3 u8 color channel values into a single u32 to be used in stuff like minifb.

```
let red = from_u8_rgb(255, 0, 0);
```

### distance
Gets the distance between 2 x and y values.

```
let distance = distance(15.0, 29.0, 243.0, 44.0);
```
*This will likely get added as methods on the point types*

### dot_product_f64
Gets the dot product of two Vec<f64> of equal length.

```
let VectorA = vec![2.0; 10];
let VectorB = vec![3.0; 10];
let product = dot_product_f64(VectorA, VectorB);
```

### matrix_mul_f64
Multiplies two Array2d<f64> together.

```
let mul_tableA = array2d!(50, 30, f64, 2.0);
let mul_tableB = array2d!(10, 50, f64, 3.0);
let matrix = matrix_mul_f64(mul_tableA, mul_tableB);
```

### Structs
todo!();