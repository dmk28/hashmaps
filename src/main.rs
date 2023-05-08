use std::collections::HashMap;



fn main() {
    //HashMaps are created empty
   let mut hash_map = HashMap::new();
   //Insert method is used to add elements in the keyword, value configuration
   hash_map.insert("Matematica", 90);
   hash_map.insert("Geografia", 75);
   hash_map.insert("Historia", 80);
   hash_map.insert("Metodologias do Sexo", 100);
   hash_map.insert("420", 0);

   match hash_map.get("Historia") {
    Some(k) => println!("O aluno cursou Historia e tirou {}", k),
    None => println!("O aluno não cursou História"),
   }

   //Remove method is used to remove elements
   hash_map.remove("420");
   hash_map.remove("Metodologias do Sexo");
   println!("O aluno estuda 420 e Metodologias do Sexo? {} {}", hash_map.contains_key("420"), hash_map.contains_key("Metodologias do Sexo"));


}
