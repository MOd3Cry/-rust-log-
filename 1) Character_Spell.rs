struct Character {
    name: String,
    hp: f64,
}

enum Spell {
    Damage(f64),
    Heal(f64),
}

impl Character {
    fn damage(&mut self, spell_damage: Spell) {
        let x: f64 = *spell_damage.unpacked();
        if self.hp >= x {
            self.hp -= x;
            println!("{} have {} hp", self.name, self.hp);
        } else {
            self.hp = 0.0;
            println!("{} DeaD", self.name);
        }
    }

    fn healing(&mut self, spell_heal: Spell) {
        let x: f64 = *spell_heal.unpacked();
        self.hp += x;
        println!("{} have {} hp", self.name, self.hp);
    }
}

impl Spell {
    fn unpacked(&self) -> &f64 {
        match self {
            Self::Damage(z) => z,
            Self::Heal(y) => y,
        }
    }
}

fn main() {
    let mut new_character: Character = Character {
        name: String::from("Vladimir"),
        hp: 895.760,
    };
    let _statistics: Spell = Spell::Damage(65.06);
    new_character.damage(_statistics);
    let _statistics: Spell = Spell::Heal(70.0);
    new_character.healing(_statistics);
    let _statistics: Spell = Spell::Damage(0.7);
    new_character.damage(_statistics);
    let _statistics: Spell = Spell::Damage(1000.0);
    new_character.damage(_statistics);
}
