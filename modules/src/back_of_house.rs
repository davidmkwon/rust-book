pub mod cook {
    pub struct Cook {
        name: String,
        years: u32,
    }

    pub fn make_meal(_meal_name: &str) {}
}

pub mod dishwash {
    pub enum DishwasherType {
        HYUNDAI,
        TOYOTA,
    }

    pub fn wash_dishes() {}
}

// add this module to this one
pub mod management;
