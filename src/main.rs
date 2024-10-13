fn main() {
    const WIDTH: usize = 30;  // Ширина конверта
    const HEIGHT: usize = 15; // Висота конверта

    let mut envelope = String::new();

    // Зовнішні межі та діагоналі
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if row == 0 || row == HEIGHT - 1 {
                // Верхня та нижня межі конверта
                envelope.push('*');
            } else if col == 0 || col == WIDTH - 1 {
                // Ліві та праві межі конверта
                envelope.push('*');
            } else if col == row * 2 || col == WIDTH - 1 - row * 2 {
                // Діагональні лінії конверта
                envelope.push('*');
            } else {
                // Порожні символи між межами та діагоналями
                envelope.push(' ');
            }
        }
        envelope.push('\n'); // Перехід на новий рядок після закінчення рядка
    }

    // Виведення зображення конверта
    print!("{}", envelope); // Виведення всього конверта в один виклик
    println!("Конверт намальовано!"); // Повідомлення про завершення
}
