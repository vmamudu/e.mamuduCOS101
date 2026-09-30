fn main() {
   let fullname = "Chibudum John Umeh";
   let department = "Software Enginneering";
   let uni = "Pan-Atlantic University";

   let mut school = "School of Science".to_string();
   //push string
   school.push_str(" and Technology");

   println!("My name is: {}", fullname);
   //check length
   println!("My name is {} characters long ", fullname.len() );
   println!("I am a student of {} Department", department);
   println!("in the {}", school );
   println!("at the {} !",uni );

}
