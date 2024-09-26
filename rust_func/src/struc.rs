struct StuName {
    name:String,
    age:u8,
    reg_no:u32,
  }
  impl StuName {
    fn func_struc(self:StuName)
    {
      println!("Student name is {}",self.name);
      println!("Student age is {}",self.age);
      println!("Student reg num is {}",self.reg_no);
    }
  }