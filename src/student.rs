

pub  struct Student{
    name:String,
    age:u8,
    roll:u8,
    grade:String,

}

impl Student {
    fn new (name:String,age:u8,roll:u8,grade:String)->Student{
        return Student{
            name,
            age,
            roll,
            grade

        };


    }

    pub fn print(&self){
        println!("name:{},age:{},roll:{},grade:{}",self.name,self.age,self.roll,self.grade);

    }



}

pub fn enter_student() -> Student {
    println!("Enter Student's name:");
    let name =scanln_fmt!("{}",String);
    println!("Enter Student's age:");
    let age =scanln_fmt!("{}",u8);
    println!("Enter Student's roll:");
    let roll =scanln_fmt!("{}",u8);
    println!("Enter Student's grade:");
    let grade =scanln_fmt!("{}",String);
    let student =Student::new(name.unwrap(),age.unwrap(),roll.unwrap(),grade.unwrap());
    return student;
}
