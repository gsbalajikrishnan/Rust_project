include!("file1.rs");
include!("file2.rs");
include!("array_file.rs");  
include!("ctrl_flow.rs");
include!("struc.rs");
fn main() 
{
  let stu_name = StuName{
  name:String::from("Balaji"),
  age:25,
  reg_no:10002,
  };
  stu_name.func_struc();
  fun_file_one();
  fun_file_two();
  //func_array();  
  func_match();
}

