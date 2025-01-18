mkdir rust_projesi
cd rust_projesi
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    let string1 = String::from("Merhaba ");
    let string2 = String::from("Dünya!");
    
    let concatenated_string = concatenate_strings(&string1, &string2);
    
    println!("Birleştirilmiş string: {}", concatenated_string);
    
    // Orijinal stringlerin hala kullanılabilir olduğunu gösterelim
    println!("İlk string: {}", string1);
    println!("İkinci string: {}", string2);
}