/*
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

*/


use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Завершуємо реалізацію функції 'compareTriplets', яка
 * приймає два масиви цілих чисел (по три оцінки) і повертає
 * масив з двох елементів: кількість виграшів для першої та другої трійки.
 */
fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut score_a = 0; // Лічильник для виграшів першої трійки
    let mut score_b = 0; // Лічильник для виграшів другої трійки

    // Порівнюємо оцінки з обох масивів
    for i in 0..3 {
        if a[i] > b[i] {
            score_a += 1; // Якщо перша трійка виграє, збільшуємо її лічильник
        } else if a[i] < b[i] {
            score_b += 1; // Якщо друга трійка виграє, збільшуємо її лічильник
        }
    }

    // Повертаємо вектор з виграшами
    vec![score_a, score_b]
}
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Створюємо файл для запису результату
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Зчитуємо першу трійку оцінок
    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Зчитуємо другу трійку оцінок
    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Порівнюємо трійки та отримуємо результати
    let result = compare_triplets(&a, &b);

    // Записуємо результати в файл
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        // Додаємо пробіл між елементами, якщо це не останній елемент
        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    // Додаємо новий рядок після запису всіх результатів
    writeln!(&mut fptr).ok();
}

// Модуль для тестування функцій
#[cfg(test)]
mod tests {
    use super::*;

    // Тестова функція для перевірки роботи compareTriplets
    #[test]
    fn test_compare_triplets() {
        // Тестовий випадок 1
        let a = vec![5, 6, 7];
        let b = vec![3, 6, 10];
        let result = compare_triplets(&a, &b);
        assert_eq!(result, vec![1, 1]); // 1 виграш для a і 1 виграш для b

        // Тестовий випадок 2
        let a = vec![1, 2, 3];
        let b = vec![3, 2, 1];
        let result = compare_triplets(&a, &b);
        assert_eq!(result, vec![1, 1]); // 1 виграш для a і 1 виграш для b

        // Тестовий випадок 3
        let a = vec![10, 20, 30];
        let b = vec![30, 20, 10];
        let result = compare_triplets(&a, &b);
        assert_eq!(result, vec![1, 1]); // 1 виграш для a і 1 виграш для b

        // Тестовий випадок 4
        let a = vec![0, 0, 0];
        let b = vec![0, 0, 0];
        let result = compare_triplets(&a, &b);
        assert_eq!(result, vec![0, 0]); // Ніхто не виграв
    }
}

/*
Функція compareTriplets:

Використовує два лічильники (score_a і score_b) для відстеження кількості виграшів для обох трійок.
Порівнює відповідні оцінки з двох масивів. Якщо оцінка з першого масиву більша, збільшує score_a; якщо менша – збільшує score_b.
Повертає вектор з результатами.
Функція main:

Читає дані з стандартного вводу, обробляє їх у вектори.
Викликає compareTriplets для порівняння трійок і записує результати у файл.

Тестова функція test_compare_triplets:
Додано кілька тестових випадків для перевірки коректності функції compareTriplets.
Кожен тест викликає compareTriplets з двома векторами, а потім використовує assert_eq!, щоб перевірити, чи повертає функція очікуваний результат.
*/