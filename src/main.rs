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

// fn input_num(prompt: &str) -> f32 { //나중에 러스트 제너릭? 으로 형 변환 알아보기
//     println!("{}", prompt);
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     input.trim().parse().expect("Please input a valid number") //return f32 number
// }

fn main() {
    // 구조체 초기화 --- (*2)
    println!("Type your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Faild to read line");
    let name = name.trim().to_string();

    let hong = Body::new(name ,80.0, 165.0);
    hong.show();


    //이름, 성별, 나이, 키, 몸무게 입력하면 맞춰서 정보제공
    //회원번호 입력해서 정보 저장
    println!("Welcome to Health Indicator");
    loop{
            println!("Type your name pls: ");
            
        

    }

}

