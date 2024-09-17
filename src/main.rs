use std::collections::HashMap;

fn main() {
    // Исходные данные
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    // Функция для вычисления интервала
    fn get_interval(temp: f64) -> String {
        let lower_bound = (temp / 10.0).floor() * 10.0;
        let upper_bound = lower_bound + 10.0;
        format!("[{:.0}, {:.0})", lower_bound, upper_bound)
    }

    // Создаем HashMap для хранения интервалов
    let mut intervals: HashMap<String, Vec<f64>> = HashMap::new();

    // Разбиваем температуры по интервалам
    for &temp in &temperatures {
        let interval = get_interval(temp);
        intervals.entry(interval).or_insert_with(Vec::new).push(temp);
    }

    // Выводим результат
    for (interval, temps) in intervals {
        println!("{}: {:?}", interval, temps);
    }
}
