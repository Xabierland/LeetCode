// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

impl Solution {
    pub fn add_digits(num: i32) -> i32 {    // La raiz digital de un numero es el residuo de dividir el numero por 9
        if num == 0 {                       // La raiz de 0 es 0 
            return 0;
        }
        if num % 9 == 0 {                   // Los numeros divisibles de forma exacta por 9 tienen raiz 9
            return 9;
        }
        num % 9                             // Si divides un numero por 9, el residuo es la raiz
    }
}