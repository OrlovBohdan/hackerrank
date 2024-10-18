
/*
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn aVeryBigSum(ar: &[i64]) -> i64 {

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = aVeryBigSum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}


*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Заверште функцію 'aVeryBigSum' нижче.
 *
 * Функція повинна повертати LONG_INTEGER.
 * Функція приймає LONG_INTEGER_ARRAY ar як параметр.
 */

// Функція, яка обчислює суму елементів масиву
fn a_very_big_sum(ar: &[i64]) -> i64 {
    // Ініціалізуємо змінну для накопичення суми
    let mut total = 0;

    // Проходимо через всі елементи масиву та додаємо їх до total
    for &value in ar {
        total += value;
    }

    // Повертаємо загальну суму
    total
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin(); // Отримуємо стандартний ввід
    let mut stdin_iterator = stdin.lock().lines(); // Локалізуємо строки вводу

    // Створюємо файл для запису результату
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Читаємо кількість елементів у масиві
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Читаємо елементи масиву та конвертуємо їх у Vec<i64>
    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    // Викликаємо функцію aVeryBigSum та зберігаємо результат
    let result = a_very_big_sum(&ar);

    // Записуємо результат у файл
    writeln!(&mut fptr, "{}", result).ok();
}

// Функція для перевірки роботи програми
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_very_big_sum() {
        let arr = vec![1000000001, 1000000002, 1000000003, 1000000004, 1000000005];
        let result = a_very_big_sum(&arr);
        assert_eq!(result, 5000000015); // Перевірка, що сума дорівнює 5000000015
    }

    #[test]
    fn test_a_very_big_sum_empty() {
        let arr: Vec<i64> = vec![];
        let result = a_very_big_sum(&arr);
        assert_eq!(result, 0); // Перевірка, що сума порожнього масиву дорівнює 0
    }

    #[test]
    fn test_a_very_big_sum_single_element() {
        let arr = vec![42];
        let result = a_very_big_sum(&arr);
        assert_eq!(result, 42); // Перевірка, що сума з одного елемента дорівнює самому елементу
    }
}

/*
Додав коментарі до кожної частини коду, щоб пояснити, що відбувається.
Функція aVeryBigSum: Реалізована функція, яка обчислює суму елементів масиву.
Тести: Додані модулі для тестування, які перевіряють правильність роботи функції aVeryBigSum.
Тести включають перевірки для масиву з кількома елементами, порожнього масиву та масиву з одним елементом.
*/

