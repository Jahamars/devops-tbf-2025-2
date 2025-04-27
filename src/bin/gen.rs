use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let mut file = File::create("rosa_test_points.txt")?;
    let mut rng = rand::thread_rng();
    
    // Параметры для формирования тестовых данных
    let n = 4.0;  // количество лепестков
    let a = 5.0;  // масштаб
    
    // Запишем параметры кривой в начало файла
    writeln!(file, "# Параметры Розы Гвидо")?;
    writeln!(file, "n {} a {}", n, a)?;
    writeln!(file, "# Формат: r theta is_on_curve (1=да, 0=нет)")?;
    
    // Генерация точек, которые находятся на кривой Розы Гвидо
    for i in 0..20 {
        let theta = i as f64 * PI / 10.0;  // равномерно распределяем углы
        let r = a * (n * theta).sin();      // точка на кривой
        
        writeln!(file, "{:.6} {:.6} 1", r, theta)?;
    }
    
    // Генерация точек, которые не находятся на кривой
    for _ in 0..20 {
        let theta = rng.gen::<f64>() * 2.0 * PI;     // случайный угол
        let r = rng.gen::<f64>() * 10.0;             // случайный радиус
        
        // Проверим, не лежит ли точка случайно на кривой
        let expected_r = a * (n * theta).sin();
        let epsilon = 1e-10;
        
        if (r - expected_r).abs() > epsilon {
            writeln!(file, "{:.6} {:.6} 0", r, theta)?;
        } else {
            // Если точка случайно оказалась на кривой, слегка изменим радиус
            writeln!(file, "{:.6} {:.6} 0", r + 0.1, theta)?;
        }
    }
    
    println!("Файл с тестовыми данными создан: rosa_test_points.txt");
    Ok(())
}
