

#[macro_use] extern crate scan_fmt;
mod student;

use scan_fmt::parse::ScanError;
use crate::student::{enter_student, Student};
use std::ops::Deref;


fn enter_record() -> Student {
    println!("Enter record");
    let st=enter_student();
    return st;

}

fn display_record(records:&Vec<Student>){
    println!("display record");
    for s in records.iter(){
        s.print();

    }

}

fn delete_record(){
    println!("delete record");

}

fn exit_app(){
    println!("bye.....");

}

fn no_op(){
    println!("no op");

}

fn display_menu() -> Result<u8, ScanError> {
    println!("1.Enter student record.");
    println!("2.Display All Students");
    println!("3.Delete Student Record.");
    println!("4.Close App.");
    let input=scanln_fmt!("{}",u8);
    return input;


}



fn main() {
   let mut running=true;
    let mut records:Vec<Student>=Vec::new();
    while(running){
        let input=display_menu();
        match input {
            Ok(val)=>{
                match val {
                    1=> {
                        let s=enter_record();
                        records.push(s);
                    },
                    2=> display_record(&records),
                    3=> delete_record(),
                    4=> {
                        exit_app();
                        running=false;
                    },
                    _ => no_op()
                }

            },
            Err(e)=>{
                println!("{}",e);
            }
        }

    }

}
