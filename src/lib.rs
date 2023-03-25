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
    ($width:expr , $height:expr , $type:ty , $default:tt) => {Array2d::<$type>::new($width, $height, $default, false)};
    ($width:expr , $height:expr , $type:ty , $default:tt ? $write_lt:tt) => {Array2d::<$type>::new($width, $height, $default, $write_lt)};
    ($width:expr , $height:expr ; $vector:tt) => {Array2d::new_from($width, $height, $vector, false)};
    ($width:expr , $height:expr ; $vector:tt ? $write_lt:tt) => {Array2d::new_from($width, $height, $vector, $write_lt)};
}
//array2d!(width, height, type, default)
//array2d!(width, height; vec<T>)
//array2d!(width, height, type, default ? bool)
//array2d!(width, height; vec<T> ? bool)



#[macro_export] macro_rules! point {
    ($x:tt, $y:tt) => {Point2::new($x, $y)};
    ($x:tt, $y:tt, $z:tt) => {Point3::new($x, $y, $z)};
}








//FUNCTIONS

//i did not make this
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}


pub fn distance(x1:&f64, y1:&f64, x2:&f64, y2:&f64) -> f64 {
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
pub fn matrix_mul_f64(multiplicand:&Array2d<f64>, multiplier:&Array2d<f64>) -> Array2d<f64> {

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

    has_lt:bool,
    lookup_table:Vec<(usize, usize)>,
}


impl<T: Clone> Array2d<T> {
    pub fn new(width:usize, height:usize, default: T, should_write_lt:bool) -> Self /*where T: Clone*/{
        let length = width * height;
        let data = vec![default; length];
        
        let mut lookup_table:Vec<(usize, usize)> = vec![];
        
        if should_write_lt {
            for i in 0..length {
                lookup_table.push((i % width, i / width));
            }
        }

        Array2d {data, width, height, length, lookup_table, has_lt:should_write_lt}
    }

    pub fn new_from(width:usize, height:usize, data:Vec<T>, should_write_lt:bool) -> Self {
        let length = width * height;
        if data.len() != length {panic!("incorrect sizes: the vector length is {} but the dimension length is {} ({} x {})",data.len(), length, width, height)}
        
        let mut lookup_table:Vec<(usize, usize)> = vec![];   

        if should_write_lt {
            for i in 0..length {
                lookup_table.push((i % width, i / width));
            }
        }

        Array2d {data, width, height, length, lookup_table, has_lt:should_write_lt}
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

    pub fn has_lt(&self) -> bool {
        self.has_lt.clone()
    }


    //misc functions
    pub fn nth_to_pos(&self, i:usize) -> (usize, usize) {
        if self.has_lt {
            self.lookup_table[i].clone()
        }else{
        (i % self.width,i / self.width)
        }

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
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2<T: num_traits::Num + Clone + Copy> {
    x:T,
    y:T,
}

impl<T: num_traits::Num + Clone + Copy> Point2<T> {
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
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<T: num_traits::Num + Clone + Copy> {
    x:T,
    y:T,
    z:T,
}

impl<T: num_traits::Num + Clone + Copy> Point3<T> {
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
}









//ENUMS
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red = 16711680,
    Green = 65280,
    Blue = 255,
    Yellow = 16776960,
    Cyan = 65535,
    Magenta = 16711935,
    Orange = 16744448,
    Blank = 0,
    White = 16777215,
}

impl Color {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}