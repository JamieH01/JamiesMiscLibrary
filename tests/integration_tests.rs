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
    fn test_array2d() {
        let mut table1 = Array2d::<u32>::new(500, 500, 15, false);
        println!("{:?} {:?}", table1.has_lt(), table1.nth(8879));

        let table2 = Array2d::<u32>::new(500, 500, 15, true);
        println!("{:?}  {:?}", table2.has_lt(), table2.nth_to_pos(8879));

        let table3 = array2d!(300, 400, u32, 15);
        let table4 = array2d!(300, 400, f64, 45.33 ? true);

        let test_vector = vec![5; 100];

        let table3 = array2d!(10, 10; test_vector);

        let mul_tableA = array2d!(50, 30, f64, 2.0);
        let mul_tableB = array2d!(10, 50, f64, 3.0);
        let matrix = matrix_mul_f64(mul_tableA, mul_tableB);
    }

    #[test]
    fn test_point() {
        let point1 = Point2::<u32>::new(15, 12);
        let point2 = point1.const_add(4);

        let macro_point2 = point!(43, 25);
        let macro_point3 = point!(23, 3, 10);
        watch!(point1, point2, macro_point2, macro_point3);
        let p1 = point!(4, 5, 6);
        let p2 = p1.const_add(1);
        assert_eq!(p2, point!(5, 6, 7));
    }

    #[test]
    fn test_color() {
        let test_color = Color::Red.value();
        println!("{:?}", test_color);

    }

    #[test]
    fn distance_test() {
        let distance = distance(15.0, 29.0, 243.0, 44.0);
        let point_a = point!(15.0, 29.0);
        let point_b = point!(243.0, 44.0);
        let point_distance = point_a.distance(&point_b);
        watch!(distance, point_distance);
    }

    #[test]
    fn dot_product_test() {
        let vectorA = vec![2.0; 10];
        let vectorB = vec![3.0; 10];
        let product = dot_product_f64(vectorA, vectorB);
    }

    #[test]
    fn try_it_test() {
        let int:u32 = 15;
        let usize_int:usize = try_it!(int);
    }
}