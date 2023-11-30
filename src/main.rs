use std::io;

struct Body{
    name : String,
    weight: f64,
    height: f64,
}

impl Body{
    fn new(weight:f64, height:f64,name: String) -> Body{
        Body { 
            name,
            weight,
            height,
            }
    }

    fn calc_bmi(&self) -> f64{
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn show(&self){
        println!("{}'s BMI is {:.2}",self.name,self.calc_bmi());
    }
}

fn input_float(ino: &str) -> f64 {
    println!("{}",ino);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().parse().expect("Please input valid number")
}

fn input_string(ino: &str) -> String {
    println!("{}",ino);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim().to_string()
}


fn main(){

    println!("Write you name, weight, height");
    
    let n = input_string("Write you name: ");
    let w = input_float("Write weight: ");
    let h = input_float("Write height: ");
    let b = Body::new(w,h,n);
    b.show();

    let hong = Body{
        name: "hong".to_string(),
        weight: 80.0,
        height:165.0,
    };
    let lim = Body{
        name: "lim".to_string(),
        weight:65.0,
        height:170.0,
    };

    println!("홍길동={:.2}",hong.calc_bmi());
    println!("임꺽정={:.2}",lim.calc_bmi());

}