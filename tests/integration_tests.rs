//#[cfg(test)]

mod integration_tests {
use jml::*;

    #[test]
    fn test_watch() {
        let var_a:usize = 15;
        let var_b = String::from("test string");
        watch!(var_a, var_b);
    }

    #[test]
    fn test_array2d_LT() {
        let table1 = Array2d::<u32>::new(500, 500, 15, false);
        println!("{:?} {:?}", table1.has_lt(), table1.nth_to_pos(8879));

        let table2 = Array2d::<u32>::new(500, 500, 15, true);
        println!("{:?}  {:?}", table2.has_lt(), table2.nth_to_pos(8879));

        let table3 = array2d!(300, 400, u32, 15);
        let table4 = array2d!(300, 400, f64, 45.33 ? true);

        let test_vector = vec![5; 100];

        let table3 = array2d!(10, 10; test_vector);
    }

    #[test]
    fn test_point() {
        let point1 = Point2::<u32>::new(15, 12);
        let point2 = point1.const_add(4);

        let macro_point2 = point!(43, 25);
        let macro_point3 = point!(23, 3, 10);
        watch!(point1, point2, macro_point2, macro_point3);
    }

    #[test]
    fn test_color() {
        let test_color = Color::Red.value();
        println!("{:?}", test_color);

    }
}