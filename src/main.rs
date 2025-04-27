use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Функция для проверки принадлежности точки кривой Розы Гвидо
fn is_point_on_rosa_curve(r: f64, theta: f64, n: f64, a: f64) -> bool {
    // Вычисляем ожидаемое значение r для данного угла theta
    let expected_r = a * (n * theta).sin();
    
    // Определяем погрешность для сравнения чисел с плавающей точкой
    let epsilon = 1e-6;
    
    // Проверяем, равно ли фактическое значение r ожидаемому с учетом погрешности
    (r - expected_r).abs() < epsilon
}

// Функция для чтения строк из файла
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    // Параметры Розы Гвидо
    let mut n = 0.0;
    let mut a = 0.0;
    let mut params_read = false;
    
    // Статистика проверки
    let mut total_points = 0;
    let mut correct_predictions = 0;
    
    // Открываем файл с тестовыми данными
    if let Ok(lines) = read_lines("rosa_test_points.txt") {
        for line in lines {
            if let Ok(line) = line {
                // Пропускаем комментарии
                if line.starts_with("#") {
                    continue;
                }
                
                // Парсим параметры кривой
                if line.starts_with("n ") && !params_read {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        n = parts[1].parse::<f64>().unwrap_or(0.0);
                        a = parts[3].parse::<f64>().unwrap_or(0.0);
                        params_read = true;
                        println!("Параметры Розы Гвидо: n = {}, a = {}", n, a);
                    }
                    continue;
                }
                
                // Парсим координаты точек и ожидаемый результат
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let r = parts[0].parse::<f64>().unwrap_or(0.0);
                    let theta = parts[1].parse::<f64>().unwrap_or(0.0);
                    let expected_on_curve = parts[2] == "1";
                    
                    // Проверяем принадлежность точки кривой
                    let is_on_curve = is_point_on_rosa_curve(r, theta, n, a);
                    
                    // Выводим результат
                    println!(
                        "Точка (r = {:.4}, θ = {:.4}): {}, ожидалось: {}", 
                        r, theta, 
                        if is_on_curve { "На кривой" } else { "Не на кривой" },
                        if expected_on_curve { "На кривой" } else { "Не на кривой" }
                    );
                    
                    total_points += 1;
                    if is_on_curve == expected_on_curve {
                        correct_predictions += 1;
                    }
                }
            }
        }
    } else {
        println!("Не удалось открыть файл с тестовыми данными");
        return Ok(());
    }
    
    // Выводим статистику
    if total_points > 0 {
        let accuracy = (correct_predictions as f64 / total_points as f64) * 100.0;
        println!("\nСтатистика:");
        println!("Всего проверено точек: {}", total_points);
        println!("Правильных предсказаний: {}", correct_predictions);
        println!("Точность: {:.2}%", accuracy);
    }
    
    Ok(())
}
