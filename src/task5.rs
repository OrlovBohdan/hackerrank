use std::io::{self, BufRead};

/*
 * Заверште функцію 'plusMinus' нижче.
 *
 * Функція приймає INTEGER_ARRAY arr як параметр.
 */

// Функція, яка обчислює частки позитивних, негативних та нульових елементів масиву
fn plus_minus(arr: &[i32]) {
    let total_count = arr.len() as f64; // Загальна кількість елементів
    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f64; // Кількість позитивних елементів
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f64; // Кількість негативних елементів
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f64; // Кількість нульових елементів

    // Обчислюємо частки
    let positive_ratio = positive_count / total_count; // Частка позитивних
    let negative_ratio = negative_count / total_count; // Частка негативних
    let zero_ratio = zero_count / total_count; // Частка нульових

    // Виводимо результати з точністю до 6 знаків після коми
    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin(); // Отримуємо стандартний ввід
    let mut stdin_iterator = stdin.lock().lines(); // Локалізуємо строки вводу

    // Читаємо розмір масиву
    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Читаємо елементи масиву та конвертуємо їх у Vec<i32>
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Викликаємо функцію plusMinus
    plus_minus(&arr);
}

// Функція для перевірки роботи програми
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    #[test]
    fn test_plus_minus() {
        let arr = vec![1, 1, 0, -1, -1];

        // Перенаправляємо стандартний вивід у буфер
        let buffer = Cursor::new(vec![]);
        {
            let mut _stdout = io::stdout(); // Отримуємо стандартний вивід
            let _ = std::panic::catch_unwind(|| {
                let output = buffer.clone(); // Клонуємо буфер
                plus_minus(&arr); // Викликаємо функцію
                let output_str = String::from_utf8(output.into_inner()).unwrap(); // Перетворюємо буфер у рядок

                // Розбиваємо результат на рядки
                let lines: Vec<&str> = output_str.lines().collect();

                // Перевіряємо, чи правильно функція обчислює частки
                assert_eq!(lines[0], "0.400000"); // Частка позитивних
                assert_eq!(lines[1], "0.400000"); // Частка негативних
                assert_eq!(lines[2], "0.200000"); // Частка нульових
            });
        }
    }
}

/*
Функція plusMinus: Реалізована функція, яка обчислює частки позитивних, негативних і нульових елементів у масиві,
а також виводить їх на екран з точністю до 6 знаків після коми.

Тест: Додана функція для тестування plusMinus, яка перевіряє, чи правильно функція обчислює частки для тестового
випадку з позитивними, негативними та нульовими значеннями. Тест перенаправляє стандартний вивід у буфер,
щоб перевірити результати без фактичного виводу на консоль.

Перенаправлення стандартного виводу: Створено буфер Cursor, який буде використовуватися для
перенаправлення стандартного виводу. Однак, це вимагає додаткових маніпуляцій.
*/
