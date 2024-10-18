/*
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simpleArraySum(ar: &[i32]) -> i32 {

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}

*/

// Дозволяємо компілятору ігнорувати попередження про невикористані функції (dead code)
#[allow(dead_code)]
fn main() {
    // Отримуємо доступ до стандартного вводу
    let stdin = io::stdin();
    // Локалізуємо ввід для читання рядків
    let mut stdin_iterator = stdin.lock().lines();

    // Створюємо файл для запису результату
    let mut fptr = File::create("output.txt").unwrap();

    // Зчитуємо перший рядок, який містить кількість елементів масиву
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Зчитуємо другий рядок, який містить масив чисел
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end() // Видаляємо пробіли в кінці рядка
        .split(' ') // Розділяємо рядок на елементи за пробілом
        .map(|s| s.to_string().parse::<i32>().unwrap()) // Перетворюємо кожен елемент на i32
        .collect(); // Збираємо результати в вектор

    // Обчислюємо суму елементів масиву
    let result = simple_array_sum(&ar);

    // Записуємо результат у файл
    writeln!(&mut fptr, "{}", result).ok();
}

// Імпортуємо необхідні модулі для роботи з файлами і ввід/вивід
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Завершуємо реалізацію функції 'simple_array_sum', яка
 * приймає масив цілих чисел як параметр і повертає їх суму.
 */
fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum() // Використовуємо ітератор для підсумовування елементів
}

// Модуль для тестування функцій
#[cfg(test)]
mod tests {
    // Імпортуємо все з основного модуля
    use super::*;

    // Тестова функція для перевірки роботи simple_array_sum
    #[test]
    fn test_simple_array_sum() {
        // Створюємо вектор з тестовими даними
        let arr = vec![1, 2, 3, 4, 5];
        // Перевіряємо, чи функція повертає правильну суму
        assert_eq!(simple_array_sum(&arr), 15);
    }
}



/*


Функція main() тепер без атрибута #[test], що дозволяє їй виконуватись як звичайна програма.
Тестова функція test_simple_array_sum розташована в окремому модулі tests, що дозволяє перевіряти правильність реалізації
simple_array_sum, не заважаючи виконанню основної програми.
*/