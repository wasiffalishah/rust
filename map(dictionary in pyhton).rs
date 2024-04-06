use std::collections::HashMap;
//like dictionary in python
fn main() {
 let mut map = HashMap::new();
 
 
 //inserting key value pasir in the map
 map.insert(0,"Hi");
 map.insert(1, "Hi2");
 println!("{:?}", map);
 
//checking values that exist against pointer of the key
 match map.get(&0){
     Some(str1) => println!("{}",str1),
     None => println!("Don't exist in map"),
 }
 
  match map.get(&1){
     Some(str) => println!("{}",str),
     None => println!("Don't exist in map"),
 }
//removing keypair with key 0
 map.remove(&0);
 println!("{:?}", map)
}
