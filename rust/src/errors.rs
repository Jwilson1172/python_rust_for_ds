//use std::io::ErrorKind;
use std::io::Command;

fn error_handling(dir: &str) -> Result<i32, Error> {
    println!("\n\n");
    let mut list_cmd = Command::new("ls");
    
    // basic error squashing to continue running code regardless of error
    // match list_cmd.current_dir(dir).status() {
    //     Ok(cmd) => Some(cmd),
    //     Err(e) => {println!("Directory not found"); None},
    // };

    // match list_cmd.current_dir(dir).status() {
    //     Ok(cmd) => Some(cmd),
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => {
    //         println!("Directory not found"); 
    //         None
    //         },
    //         _ => panic!("an unexspected error has occured")
    //     },
    // };

    // list_cmd.current_dir(dir).status().unwrap();
    list_cmd.current_dir(dir).status()?;

    println!("\n\n");
    return Ok(1);
}
