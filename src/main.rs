use std::string::String;
use std::io;

fn main() {
    println!("Добро пожаловать в мрачное будущее!");

    let world = loop {
        let mut input = String::new();
        println!("Выбери свой родной мир: ");
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        //print!("Input was: {}", input);
        match input.trim().parse::<u8>() {
            Err(err) => {
                println!("{}", err);
                println!("Неверный параметр. Введите число от 1 до 100.");
                continue;
            },
            Ok(seed) => {
                match generate_world(seed) {
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    },
                    Ok(world) => break world,
                }
            },
        };
    };

    //let world = World {name: String::from("Дикий мир")};
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
        Err(String::from("Число не может быть нулём!"))
    }
}
