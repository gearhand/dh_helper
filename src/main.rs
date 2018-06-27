use std::string::String;

fn main() {
    println!("Добро пожаловать в мрачное будущее!");

    print!("Выбери свой родной мир: ");
    // TODO: read numbers from stdin
    // TODO: generate world from number

    let world = World {name: String::from("Дикий мир")};
    println!("{}", world.name);

    println!("Вы можете сгенерировать характеристики с помощью генератора случайных чисел \
        или автоматически.");

    println!("Выберите свою профессию. Для вашего персонажа доступны:");

    println!("Навыки и стартовая экипировка:");

    println!("Готово! Слава Императору!");
}

struct World {
    name: String,
}

fn generate_world(seed: u8) -> Result<World, String> {
    if seed > 100 {
        Err(String::from("Seed value too high"))
    } else if seed > 95 {
        Ok(World {name: String::from("Очищеный разум")})
    } else if seed > 85 {
        Ok(World {name: String::from("Благородная кровь")})
    } else if seed > 75 {
        Ok(World {name: String::from("Схола Прогениум")})
    } else if seed > 65 {
        Ok(World {name: String::from("Мир-Кузница")})
    } else if seed > 55 {
        Ok(World {name: String::from("Рождённый в Пустоте")})
    } else if seed > 35 {
        Ok(World {name: String::from("Имперский мир")})
    } else if seed > 15 {
        Ok(World {name: String::from("Мир-Улей")})
    } else if seed > 0 {
        Ok(World {name: String::from("Дикий мир")})
    } else {
        Err(String::from("Seed value could not be zero"))
    }
}
