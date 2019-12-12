pub fn run (){
    let person = Person {
        name: String::from("Jaime"),
        age: 23
    };

    println!("can {} speak? {}", person.name, person.canSpeak());
}

struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    fn speak(&self);
    fn canSpeak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self){
        println!("hello my name is {}", self.name);
    }

    fn canSpeak(&self)->bool{
        if self.age > 18 {
            return true;
        } return false;
    }
}