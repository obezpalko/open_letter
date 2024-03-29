use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    const MAX_PHRASES: usize = 8;
    let mut num_phrases: usize = rng.gen_range(3, MAX_PHRASES);
    let mut used_indexes: [usize; MAX_PHRASES] = [99999; MAX_PHRASES];
    let parts = [
        "Маменька, вышлите деньги.",
        "Папенька, вышлите деньги.",
        "Вышлите деньги.",
        "Дождь льет, как из ведра.",
        "От комаров нѣт спасенья.",
        "Обокрали дачу.",
        "Дача с протекціей.",
        "Обобрал лавочник.",
        "Погода превосходная.",
        "Трескучій мороз.",
        "Голова трещит с позавчерашняго.",
        "Изжога - нѣвероятная.",
        "Клопы жгут.",
        "Зубы ноют.",
        "Тощища анаѳемская.",
        "Вы — ангел.",
        "Вчера я видѣл вас во снѣ.",
        "Ваши глаза — пулеметы.",
        "Высылают.",
        "Прислуга грубит.",
        "Оштрафован за ношеніе ножа.",
        "Оштрафован за ношеніе вилки.",
        "Выпустили под залог.",
        "Дайте взятку.",
        "Возьмите и молчите.",
        "Продулся в прах.",
        "Дѣти ревут.",
        "Дворник выселяет.",
        "Завтра к мировому.",
        "Хоть рыло въ крови, а наша взяла.",
        "Дѣла плохи.",
        "Питаемся сѣном.",
        "Хоронили писателя.",
        "Есть раненые.",
        "Слава Богу, только побили.",
        "Печи дымят.",
        "Ѣм, ѣм, а аппетита нѣт.",
        "Пьем ваше здоровье.",
        "А поллиція у нас вѣжливая.",
        "Ошикали.",
        "Облила сѣрной кислотой.",
        "Печень не в порядкѣ.",
        "Сидѣть больно…",
        "Вспрыснули.",
        "Всыпали.",
        "На двадцать лѣт.",
        "Прошу в моей смерти никого не винить.",
        "Зарѣзал математик.",
        "Поступил в авіаторы.",
        "Поступил в экспропріаторы.",
        "Пишет стихи.",
        "У наших сосѣдей есть граммофон.",
        "Всегда ваш…",
    ];

    let total_phrases = parts.len();
    loop {
        let index: usize = loop {
            let idx = rng.gen_range(0, total_phrases);
            if !(used_indexes.contains(&idx)) {
                used_indexes[num_phrases] = idx;
                break idx
            }
        };
        print!("{} ", parts[index]);
        num_phrases -= 1;
        if num_phrases <= 0 {break;}
    }
    println!("");
}
