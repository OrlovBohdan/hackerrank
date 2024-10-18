use std::io::{self, BufRead};

/*
 * Заверште функцію 'staircase' нижче.
 *
 * Функція приймає INTEGER n як параметр.
 */

// Функція, яка малює сходи з символів '#'
fn staircase(n: i32) -> String {
    let mut result = String::new(); // Зберігатимемо результат
    for i in 1..=n {
        // Додаємо пробіли перед символами '#'
        let spaces = " ".repeat((n - i) as usize); // Кількість пробілів
        let hashes = "#".repeat(i as usize); // Кількість символів '#'
        result.push_str(&format!("{}{}\n", spaces, hashes)); // Додаємо рядок до результату
    }
    result // Повертаємо результат
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin(); // Отримуємо стандартний ввід
    let mut stdin_iterator = stdin.lock().lines(); // Локалізуємо строки вводу

    // Читаємо розмір сходів
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Викликаємо функцію staircase
    let output = staircase(n);
    print!("{}", output); // Виводимо результат
}

// Функція для перевірки роботи програми
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let n = 5;
        let expected_output = "    #\n   ##\n  ###\n ####\n#####\n"; // Очікуваний вихід

        // Викликаємо функцію staircase
        let output = staircase(n); // Отримуємо результат

        // Перевіряємо, що вивід відповідає очікуваному
        assert_eq!(output, expected_output);
    }
}

/*
Функція staircase: Реалізована функція, яка малює сходи з символів #. Кількість пробілів зменшується,
а кількість символів # збільшується в кожному рядку.
Тест: Додана функція для тестування staircase, яка перевіряє, чи правильно функція малює сходи для тестового випадку.
Вивід порівнюється з очікуваним.

Зміна типу функції staircase: Тепер вона повертає рядок, що містить сходи.
Використання push_str: Додаємо кожен рядок до результату.
Тестування: Тест просто порівнює вихід з очікуваним рядком.
*/