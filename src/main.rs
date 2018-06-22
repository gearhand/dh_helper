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
    } else {
        Ok(World {name: String::from("Дикий мир")})
    }
}
