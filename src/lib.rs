/*
Uso de pub en metodo de libreria:
debe usarse para que la funcion/metodo sea accesible desde fuera del archivo.
*/
pub fn say_hello(){
    println!("Hello World!")
}

pub fn print_num(limit: u8){
    let numbers = generate_numbers(limit);

    show_numbers(&numbers);
}

//Uso de slices para ejecutar una funcion iteradora con arreglos y vectores simultaneamente
//Si un parametro de funcion se expresa como referencia, el arg de invocacion debe expresarse igual
fn show_numbers (numbers: &[u8]) {
    for num in numbers{
        println!("{:?}", num)
    }
}

//Al establecer un valor de retorno, este valor puede expresarse sin punto y coma
fn generate_numbers (number: u8) -> Vec<u8> {
    let mut sequence = Vec::new();
    
    //El asignador ..= declara un rango de numeros incluyendo ambos limites del rango.
    //Sin el =, el rango solo incluiria el limite izquierdo del rango
    for n in 1..=number {
        sequence.push(n);
    }

    sequence
}