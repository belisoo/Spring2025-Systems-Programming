fn intro_to_idea(){
    
    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
     }

    impl Rectangle {
         fn get_area(&self) -> f64 {
             self.width * self.height
         }
    }


    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
         fn get_area(&self) -> f64 {
             self.radius * self.radius * 3.14 as f64
         }
     }

     let rec = Rectangle {width:5.0,height:8.0};
     let circle = Circle {radius: 5.0};

     println!("Area of a rectangle {}", rec.get_area());
     println!("Area of a circle {}", circle.get_area());

    // let shapes: Vec<????> = vec![rec, circle]; 
    // unfortunately doesn't work
}