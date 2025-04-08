#[derive(Debug)]
enum Insurance {
    House,
    Car,
    Life
}

#[derive(Debug)]
struct Person {
    name:String,
    insurances:Vector<Insurance>
}

impl Person{
    fn new(n:String) -> Person {
        Person{
            name:n,
            insurances:vec![],
        }
    }
} 

fn add_insurance(&mut self, i:Insurance){
    self.insurances.push(i);
}

fn show_insurances(&self){
    println!("Hey my name is {}. I have a next type of insurances", self.name)
    for i in &self.insurances.iter(){
        println!("{:?}", i);
    }
}

fn main() {


}