// use std::fmt;
use std::io;

// 키와 몸무게 데이터를 가지는 구조체 --- (*1)
struct Body {
    name: String,
    weight: f64,
    height: f64,
}

impl Body {
    fn new(name: String, weight: f64, height: f64) -> Body {
        Body {
            name,
            weight,
            height,
        }
    }
    // BMI를 계산하는 함수 --- (*4)
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }
    fn show(&self) {
        println!("The {}'s BMI is {:.1}", self.name, self.calc_bmi());
    }
    
}


fn main() {
    // 구조체 초기화 --- (*2)
    println!("Type your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Faild to read line");
    let name = name.trim().to_string();

    //let name = "ho".to_string();
    let hong = Body::new(name ,80.0, 165.0);
    hong.show();

    // 함수 호출 --- (*3)
    // println!("홍길동 = {:.2}", calc_bmi(&hong));
    // println!("임꺽정 = {:.1}", calc_bmi(&lim));
}

