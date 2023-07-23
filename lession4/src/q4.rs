trait Animal {
    fn eat(&self);
}
trait Bird {
    fn fly(&self);
}
trait FlyAnimal: Animal + Bird {
    fn sleep(&self);
}
struct Cat {
}
struct Eagle {
}

impl Animal for Cat {
    fn eat(&self) {
        println!("cat eat");
    }
}

impl Animal for Eagle {
    fn eat(&self) {
        println!("eagle eat");
    }
}
impl FlyAnimal for Eagle {
    fn sleep(&self) {
        println!("eagle sleep");
    }
}
impl Bird for Eagle {
    fn fly(&self) {
        println!("eagle fly");
    }
}

impl Cat {
    fn jump(&self) {
        println!("jump");
    }
}
struct Worm {
}

impl Worm {
    fn crawl(&self) {
        println!("worm crawl");
    }
}
impl Eagle {
    fn hover(&self) {
        println!("eagle hover");
    }
}

enum E<A:Animal, F:FlyAnimal> {
    Cat(A),
    Eagle(F),
    Caterpillar(Worm),
    Num(i32),
    Name(String)
}

type FlyAnimalListDyn = Vec<Box<dyn FlyAnimal>>;
type FlyAnimalList<F:FlyAnimal> = Vec<F>;
type AnimalList<A:Animal> = Vec<A>;
type EnumList = Vec<E<Cat, Eagle>>;


fn list_fly_animal_list(fly_animal_list: FlyAnimalList<Eagle>) {
    for flyAnimal in fly_animal_list {
        flyAnimal.fly();
        flyAnimal.eat();
    }
}

fn list_fly_animal_list_dyn(fly_animal_list_dyn : Vec<Box<dyn FlyAnimal>>) {
    for flyAnimal in fly_animal_list_dyn {
        flyAnimal.eat();
        flyAnimal.fly();
    }
}

fn list_animal_list(animal_list : AnimalList<Cat>) {
    for animal in animal_list {
        animal.jump()
    }
}

fn list_enum_list(enum_list : EnumList) {
    for e in enum_list {
        match e {
            E::Cat(c) => c.jump(),
            E::Eagle(f) => f.fly(),
            E::Caterpillar(c) => c.crawl(),
            _ => println!("cannot found")
        }
    }
}

pub fn do_q4() {
    let c : E<Cat, Eagle> = E::Cat(Cat{});
    let e : E<Cat, Eagle> = E::Eagle(Eagle{});
    let w : E<Cat, Eagle> = E::Caterpillar(Worm{});
    let mut fly_list : FlyAnimalList<Eagle> = Vec::new();
    let mut fly_list_dyn : FlyAnimalListDyn = Vec::new();

    let eagle : Option<Eagle> = match e {
        E::Eagle(f) => Some(f),
        _ => None
    };

    fly_list.push(eagle.unwrap());
    fly_list_dyn.push( Box::new(Eagle{}));

    list_fly_animal_list(fly_list);
    list_fly_animal_list_dyn(fly_list_dyn);
}
