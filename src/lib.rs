use minifb::*;

//MACROS
#[allow(unused_macros)]
#[macro_export] macro_rules! watch {
    ($($item:tt),+) => {
        //print!("\x1B[2J");
        $(
        let name = stringify!($item);
        println!("{}: {:?}", name, $item);
        )+
    };

    ($($name:tt : $item:tt),+) => {
    //print!("\x1B[2J");
        $(
        println!("{}: {:?}", $name, $item);
        )+
    }; 
}


#[macro_export] macro_rules! array2d {
    ($width:expr , $height:expr , $type:ty , $default:tt) => {Array2d::<$type>::new($width, $height, $default)};
    ($width:expr , $height:expr ; $vector:tt) => {Array2d::new_from($width, $height, $vector)};
}
//array2d!(width, height, type, default)
//array2d!(width, height; vec<T>)
//array2d!(width, height, type, default ? bool)
//array2d!(width, height; vec<T> ? bool)



#[macro_export] macro_rules! point {
    ($x:expr, $y:expr) => {Point2::new($x, $y)};
    ($x:expr, $y:expr, $z:expr) => {Point3::new($x, $y, $z)};
}


#[macro_export] macro_rules! try_it {
    ($expr:expr) => {$expr.try_into().unwrap()}
}


#[macro_export] macro_rules! vel {
    (! $dir:expr, $mag:expr) => {Velocity::new( ($dir as f64).to_radians(), $mag)}; //degrees
    (? $dir:expr, $mag:expr) => {Velocity::new($dir, $mag)}; //radians

}

#[macro_export] macro_rules! keys {
    ($state:tt, {$($key:pat => $code:block),*}) => {
        let keys: Vec<Keycode> = $state.get_keys();
        for key in keys.iter() {   
            match key {
                $($key => {$code}),*,
            }
        }

    };
}



//FUNCTIONS

//i did not make this
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}


pub fn distance(x1:f64, y1:f64, x2:f64, y2:f64) -> f64 {
((x2-x1).powi(2)+(y2-y1).powi(2)).sqrt()
}


pub fn dot_product_f64(multiplicand:Vec<f64>, multiplier:Vec<f64>) -> f64 {
    if multiplicand.len() != multiplier.len() {panic!("incompatable tables: the multiplicand len is {} but the multiplier len is {}",multiplicand.len(),multiplier.len())}

    let mut output:f64 = 0.0;

    for i in 0..multiplicand.len() {
        output += multiplicand[i] * multiplier[i]
    }



    output
}

//make this and more matrix stuff a part of the struct
pub fn matrix_mul_f64(multiplicand:Array2d<f64>, multiplier:Array2d<f64>) -> Array2d<f64> {

    if multiplicand.width != multiplier.height {panic!("incompatable tables: the width of the multiplicand is {} but the height of the multiplier is {}", multiplicand.width, multiplier.height)}

    let mut output = array2d!(multiplier.width(), multiplicand.height(), f64, 0.0);

    for i in 0..output.len() {
        let pos = output.nth_to_pos(i);
        let product = dot_product_f64(multiplicand.get_row(pos.1), multiplier.get_column(pos.0));
        output.set_nth(i, product);
    }

    output
}








//STRUCTS
#[derive(Clone, Debug, PartialEq)]
pub struct Array2d<T> {
    //data
    pub data: Vec<T>,

    //read-only properties, grab with coresponding methods
    width:usize,
    height:usize,
    length:usize,
}


impl<T: Clone> Array2d<T> {
    pub fn new(width:usize, height:usize, default: T) -> Self /*where T: Clone*/{
        let length = width * height;
        let data = vec![default; length];

        Array2d {data, width, height, length}
    }

    pub fn new_from(width:usize, height:usize, data:Vec<T>) -> Self {
        let length = width * height;
        if data.len() != length {panic!("incorrect sizes: the vector length is {} but the dimension length is {} ({} x {})",data.len(), length, width, height)}
        
        Array2d {data, width, height, length}
    }

    //core functions
    pub fn nth(&self, i:usize) -> T {       
        
        self.data[i].clone()
    
    }

    pub fn pos(&self, x:usize, y:usize) -> T {
        
        if x >= self.width {panic!("index out of bounds: the width is {} but x is {}",self.width, x)}
        if y >= self.height {panic!("index out of bounds: the height is {} but y is {}",self.height, y)}


        let i = (y * self.width) + x;           
    
        self.data[i].clone()
    
    }


    pub fn set_nth(&mut self, i:usize, value:T) {
            
        self.data[i] = value;

    }

    pub fn set_pos(&mut self, x:usize, y:usize, value:T) {

        if x >= self.width {panic!("index out of bounds: the width is {} but x is {}",self.width, x)}
        if y >= self.height {panic!("index out of bounds: the height is {} but y is {}",self.height, y)}

        let i = (y * self.width) + x;
        
        self.data[i] = value;

    }


    //property grabs
    pub fn width(&self) -> usize {
        self.width.clone()
    }

    pub fn height(&self) -> usize {
        self.height.clone()
    }

    pub fn len(&self) -> usize {
        self.length.clone()
    }



    //misc functions
    pub fn nth_to_pos(&self, i:usize) -> (usize, usize) {
        
        (i % self.width,i / self.width)

    }

    pub fn pos_to_nth(&self, x:usize, y:usize) -> usize {

        (y * self.width) + x
        
    }


    pub fn iter(&self) -> std::slice::Iter<T> {
    
        self.data.iter()
    
    }

    pub fn get_row(&self, row:usize) -> Vec<T> {

        let mut output:Vec<T> = vec![];

        for i in 0..self.width {
            output.push(self.pos(i, row))
        }
    
        output
    }

    pub fn get_column(&self, column:usize) -> Vec<T> {

        let mut output:Vec<T> = vec![];

        for i in 0..self.height {
            output.push(self.pos(column, i))
        }
    
        output
    }


    pub fn pop_data(self) -> Vec<T> {
        self.data
    }


    pub fn is_bounded(&self, x:usize, y:usize) -> bool {
        if x < self.width && y < self.height {return true}else{return false}
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2<T:num_traits::Num +  Clone + Copy> {
    x:T,
    y:T,
}

impl<T:num_traits::Num +  Clone + Copy> Point2<T> where f64: From<T> {
    pub fn new(x:T, y:T) -> Self {
        Point2 {x, y}
    }

    pub fn add(&self, addend:Point2<T>) -> Self {
        let new_x = self.x + addend.x;
        let new_y = self.y + addend.y;
        Point2 {x:new_x, y:new_y}
    }
    pub fn sub(&self, subtrahend:Point2<T>) -> Self {
        let new_x = self.x - subtrahend.x;
        let new_y = self.y - subtrahend.y;
        Point2 {x:new_x, y:new_y}
    }
    pub fn mul(&self, multiplier:Point2<T>) -> Self {
        let new_x = self.x * multiplier.x;
        let new_y = self.y * multiplier.y;
        Point2 {x:new_x, y:new_y}
    }
    pub fn div(&self, divisor:Point2<T>) -> Self {
        let new_x = self.x / divisor.x;
        let new_y = self.y / divisor.y;
        Point2 {x:new_x, y:new_y}
    }



    pub fn const_add(&self, addend:T) -> Self {
        let new_x = self.x + addend;
        let new_y = self.y + addend;
        Point2 {x:new_x, y:new_y}
    }
    pub fn const_sub(&self, subtrahend:T) -> Self {
        let new_x = self.x - subtrahend;
        let new_y = self.y - subtrahend;
        Point2 {x:new_x, y:new_y}
    }
    pub fn const_mul(&self, multiplier:T) -> Self {
        let new_x = self.x * multiplier;
        let new_y = self.y * multiplier;
        Point2 {x:new_x, y:new_y}
    }    
    pub fn const_div(&self, divisor:T) -> Self {
        let new_x = self.x / divisor;
        let new_y = self.y / divisor;
        Point2 {x:new_x, y:new_y}
    }


    pub fn distance(&self, p2:&Self) -> f64 {
    let x1:f64 = self.x.try_into().unwrap();
    let y1:f64 = self.y.try_into().unwrap();
    let x2:f64 = p2.x.try_into().unwrap();
    let y2:f64 = p2.y.try_into().unwrap();

    ( (x2-x1).powi(2) + (y2-y1).powi(2) ).sqrt()
    }

    pub fn as_velocity(&self) -> Velocity {
        let fx:f64 = try_it!(self.x);
        let fy:f64 = try_it!(self.y);

        let mag = ( fx.powi(2) + fy.powi(2) ).sqrt();
        let dir = fy.atan2(fx);

        Velocity {mag, dir}
    }

}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<T: num_traits::Num + Clone + Copy> {
    x:T,
    y:T,
    z:T,
}

impl<T: num_traits::Num + Clone + Copy> Point3<T> where f64: From<T>  {
    pub fn new(x:T, y:T, z:T) -> Self {
        Point3 {x, y, z}
    }

    pub fn add(&self, addend:Point3<T>) -> Self {
        let new_x = self.x + addend.x;
        let new_y = self.y + addend.y;
        let new_z = self.z + addend.z;
        Point3 {x:new_x, y:new_y, z:new_z}
    }
    pub fn sub(&self, subtrahend:Point3<T>) -> Self {
        let new_x = self.x - subtrahend.x;
        let new_y = self.y - subtrahend.y;
        let new_z = self.z - subtrahend.z;
        Point3 {x:new_x, y:new_y, z:new_z}
    }
    pub fn mul(&self, multiplier:Point3<T>) -> Self {
        let new_x = self.x * multiplier.x;
        let new_y = self.y * multiplier.y;
        let new_z = self.z * multiplier.z;
        Point3 {x:new_x, y:new_y, z:new_z}
    }
    pub fn div(&self, divisor:Point3<T>) -> Self {
        let new_x = self.x / divisor.x;
        let new_y = self.y / divisor.y;
        let new_z = self.z / divisor.z;
        Point3 {x:new_x, y:new_y, z:new_z}
    }



    pub fn const_add(&self, addend:T) -> Self {
        let new_x = self.x + addend;
        let new_y = self.y + addend;
        let new_z = self.z + addend;
        Point3 {x:new_x, y:new_y, z:new_z}
    }
    pub fn const_sub(&self, subtrahend:T) -> Self {
        let new_x = self.x - subtrahend;
        let new_y = self.y - subtrahend;
        let new_z = self.z - subtrahend;
        Point3 {x:new_x, y:new_y, z:new_z}
    }
    pub fn const_mul(&self, multiplier:T) -> Self {
        let new_x = self.x * multiplier;
        let new_y = self.y * multiplier;
        let new_z = self.z * multiplier;
        Point3 {x:new_x, y:new_y, z:new_z}
    }    
    pub fn const_div(&self, divisor:T) -> Self {
        let new_x = self.x / divisor;
        let new_y = self.y / divisor;
        let new_z = self.z / divisor;
        Point3 {x:new_x, y:new_y, z:new_z}
    }


    pub fn distance(&self, p2:&Self) -> f64 {
    //this is kinda stupid i think but idc
    let x1:f64 = try_it!(self.x);
    let y1:f64 = try_it!(self.y);
    let z1:f64 = try_it!(self.z);
    let x2:f64 = try_it!(p2.x);
    let y2:f64 = try_it!(p2.y);
    let z2:f64 = try_it!(p2.z);

    ( (x2-x1).powi(2) + (y2-y1).powi(2) + (z2-z1).powi(2) ).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dir:f64,
    pub mag:f64,
}

impl Velocity {
    pub fn new(dir:f64, mag:f64) -> Self { Velocity {dir, mag} }

    pub fn as_point(&self) -> Point2<f64> {
        point!(
        self.mag * self.dir.cos(),
        self.mag * self.dir.sin()
        )
    }


    pub fn add(&self, addend:Velocity) -> Self {
        let new_dir = self.dir + addend.dir;
        let new_mag = self.mag + addend.mag;
        Velocity {dir:new_dir, mag:new_mag}
    }
    pub fn sub(&self, subtrahend:Velocity) -> Self {
        let new_dir = self.dir - subtrahend.dir;
        let new_mag = self.mag - subtrahend.mag;
        Velocity {dir:new_dir, mag:new_mag}
    }
    pub fn mul(&self, multiplier:Velocity) -> Self {
        let new_dir = self.dir * multiplier.dir;
        let new_mag = self.mag * multiplier.mag;
        Velocity {dir:new_dir, mag:new_mag}
    }
    pub fn div(&self, divisor:Velocity) -> Self {
        let new_dir = self.dir / divisor.dir;
        let new_mag = self.mag / divisor.mag;
        Velocity {dir:new_dir, mag:new_mag}
    }
}


pub struct WindowContainer {
    pub buffer:Array2d<u32>,
    pub window:Window,
    
    //properties
    pub width:usize,
    pub height:usize,
    pub bg_color:u32,
}

impl WindowContainer {
    pub fn new(width:usize, height:usize, name:&str, bg_color:u32) -> Self {
        let buffer = array2d!(width, height, u32, bg_color);
        let window = Window::new(name, width, height, WindowOptions::default()).unwrap();

        WindowContainer {buffer, window, width, height, bg_color}
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer.data, self.width, self.height).unwrap();
    }

}



//ENUMS
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red     = 16711680,
    Green   = 65280,
    Blue    = 255,
    Yellow  = 16776960,
    Cyan    = 65535,
    Magenta = 16711935,
    Orange  = 16744448,
    Black   = 0,
    Grey    = 4934475,
    White   = 16777215,
}

impl Color {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}