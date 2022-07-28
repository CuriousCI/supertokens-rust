pub trait Recipe<'a> {
    fn recipe_id(&self) -> &'a str;
}

// pub struct Recipe
