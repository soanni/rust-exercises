enum Silverware {
    Spoon,
    Fork,
    Glass,
    Cup,
}

fn main() {
    use String as S;

    let s = S::from("hello world!");

    println!("{}", s);

    get_silverware(&Silverware::Cup);
}

fn get_silverware(item: &Silverware) {
    use Silverware::{
        Cup as Чашка, Fork as Вилка, Glass as Стакан, Spoon as Ложка
    };

    match item {
        Ложка => println!("got a ложка"),
        Вилка => println!("got a вилка"),
        Стакан => println!("got a стакан"),
        Чашка => println!("got a чашка"),
    };
}
