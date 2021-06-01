use std::env;

struct TodoItem{
   name:String,
   comepleted:char
} 

impl TodoItem{

   fn new(name:String) -> TodoItem{
      return TodoItem{
      name:name,
       comepleted:' '
      };
   }
}

struct TodoList{

   list: Vec<TodoItem>
}
  impl TodoList
{
   fn new()->TodoList{
      TodoList{
      list: Vec::new()}
  }
  
  
  fn add_to_list(&mut self,name:String){
   let todo_item= TodoItem::new(name);  
   self.list.push(todo_item);
  }
  fn print(&self){
     for (index,item) in self.list.iter().enumerate(){
        println!("{ } -[{ }] - { }",index, item.comepleted,item.name);
     }
  } 
  fn mark(&mut self,index: usize) {
    if self.list[index].comepleted ==' ' {
       self.list[index].comepleted ='x';
    }else
    {
   self.list[index].comepleted =' '
   }
  }
  fn remove_task(&mut self, index: usize)
  {
   self.list.remove(index);
  }
}

enum Command{

   Get,
   Add(String),
   Done(usize),
   Remove(usize)
}

fn main(){
 let arguments: Vec<String> = env::args().collect();
 let command = match arguments[1].as_str(){
    
   "get"=>Command::Get,
   "add"=>Command::Add(arguments[2].clone()),
   "done"=>Command::Done(arguments[2].parse().unwrap()),
   "remove"=>Command::Remove(arguments[2].parse().unwrap()),
   
    _=> panic!("invalid comand")
   };
  
 
 
 let mut todo_list = TodoList::new();
 todo_list.add_to_list("welcome to rust".to_string());
 todo_list.add_to_list("refer the documentation whenever getting erros".to_string());
    
    match command{ 

       Command::Get=>todo_list.print(),
       Command::Add(task ) =>{
         todo_list.add_to_list(task);
         todo_list.print();
       },
       Command::Done(index)=> {
       todo_list.mark(index);
       todo_list.print();
     },
       Command::Remove(index)=>{
       todo_list.remove_task(index);
       todo_list.print();
     }
   }
}