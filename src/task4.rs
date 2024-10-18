use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Заверште функцію 'diagonalDifference' нижче.
 *
 * Функція повинна повертати INTEGER.
 * Функція приймає 2D_INTEGER_ARRAY arr як параметр.
 */

// Функція, яка обчислює різницю між сумами діагоналей квадратної матриці
fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum = 0; // Сума основної діагоналі
    let mut secondary_diagonal_sum = 0; // Сума побічної діагоналі
    let n = arr.len(); // Розмір матриці (кількість рядків/стовпців)

    // Проходимо через матрицю та обчислюємо суми діагоналей
    for i in 0..n {
        primary_diagonal_sum += arr[i][i]; // Додаємо елемент основної діагоналі
        secondary_diagonal_sum += arr[i][n - 1 - i]; // Додаємо елемент побічної діагоналі
    }

    // Обчислюємо абсолютну різницю між сумами
    (primary_diagonal_sum - secondary_diagonal_sum).abs() // Повертаємо абсолютне значення різниці
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin(); // Отримуємо стандартний ввід
    let mut stdin_iterator = stdin.lock().lines(); // Локалізуємо строки вводу

    // Створюємо файл для запису результату
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Читаємо розмір матриці
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Ініціалізуємо матрицю
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    // Заповнюємо матрицю з вводу
    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));
        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    // Викликаємо функцію diagonal_difference та зберігаємо результат
    let result = diagonal_difference(&arr);

    // Записуємо результат у файл
    writeln!(&mut fptr, "{}", result).ok();
}

// Функція для перевірки роботи програми
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let arr = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];
        let result = diagonal_difference(&arr);
        assert_eq!(result, 2); // Перевірка, що різниця дорівнює 2
    }

    #[test]
    fn test_diagonal_difference_zero() {
        let arr = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        let result = diagonal_difference(&arr);
        assert_eq!(result, 0); // Перевірка, що різниця дорівнює 0
    }

    #[test]
    fn test_diagonal_difference_single_element() {
        let arr = vec![
            vec![5],
        ];
        let result = diagonal_difference(&arr);
        assert_eq!(result, 0); // Перевірка, що різниця з одного елемента дорівнює 0
    }
}


/*
Функція diagonalDifference: Реалізована функція, яка обчислює різницю між сумами основної та побічної діагоналей квадратної матриці.
Тести: Додані модулі для тестування, які перевіряють правильність роботи функції diagonalDifference.
Тести включають випадки з нормальною матрицею, матрицею, що складається з нулів, та матрицею з одного елемента.
*/