use std::string::String;
use std::io;
use std::fs::File;
use std::env::args;

extern crate serde;
//extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

fn main() {


    let arguments: Vec<String> = args().collect();
    println!("Arguments:  {} {}", arguments[0], arguments[1]);
    //let worlds : Vec<World> = serde_json::from_reader(File::open(arguments[1].clone()).unwrap()).expect("Failed to load config");
    let worlds : Vec<World> = serde_yaml::from_reader(File::open(arguments[1].clone()).unwrap()).expect("Failed to load config");
    //println!("Worlds: {:?}", worlds);
    //return;

    println!("Добро пожаловать в мрачное будущее!");

    let world: &World = loop {
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
                match filter_world(seed, &worlds) {
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

    println!("Выберите свою профессию. Для вашего персонажа доступны: {:?}", world.classes);

    println!("Навыки и стартовая экипировка:");

    println!("Готово! Слава Императору!");
}

#[derive (Serialize, Deserialize, Debug)]
struct World {
    token: String,
    name: String,
    classes: Vec<String>,
    skills: Vec<String>,
    features: Vec<String>,
}

struct Character {
    description: String,
}

/*fn generate_world(seed: u8, worlds: &Vec<World>) -> Result<World, String> {
    if seed > 100 {
        Err(String::from("Значение параметра слишком велико!"))
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
}*/

fn generate_token(seed: u8) -> Result<String, String> {
    if seed > 100 {
        Err(String::from("Значение параметра слишком велико!"))
    } else if seed > 95 {
        Ok("E".to_owned()) // Empty mind
    } else if seed > 85 {
        Ok("N".to_owned()) // Noble blood
    } else if seed > 75 {
        Ok("S".to_owned()) // Schola Progenium
    } else if seed > 65 {
        Ok("F".to_owned()) // Forge world
    } else if seed > 55 {
        Ok("V".to_owned()) // Born in Void
    } else if seed > 35 {
        Ok("I".to_owned()) // Imperium world
    } else if seed > 15 {
        Ok("H".to_owned()) // Hive
    } else if seed > 0 {
        Ok("W".to_owned()) // Wild world
    } else {
        Err(String::from("Число не может быть нулём!"))
    }
}

fn filter_world(seed: u8, config: &Vec<World>) -> Result<&World, String> {
    let token = match generate_token(seed) {
        Ok(t) => t,
        Err(e) => return Err(e)
    };
    let world = config.iter().find(|w| {
        w.token == token
    });
    match world {
        Some(w) => Ok(w),
        None => Err(format!("No token {} in current configuration!", token))
    }
}
