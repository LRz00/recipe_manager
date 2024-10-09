use crate::recipe::Recipe;

use std::fs;

pub struct RecipeManager{
recipes: vec<Recipe>,
next_id: u32,
}

impl RecipeManager {
    pub fn new () -> Self {
        RecipeManager{
            recipes: Vec::new(),
            next_id: 1,
        }
    } 

    pub fn add_recipe(&mut self, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> u32{
        let id = self.next_id;
        
        self.recipes.push(Recipe::new(id, name, ingredients, instructions, servings));

        self.next_id += 1;

        id;
    }   

    pub fn get_all_recipes(&self) -> &Vec<Recipe>{
        &self.recipes;
    }

    pub fn get_recipe(&self, id: i32) -> Option<&Recipe>{
        self.recipes.iter().find(|r| r.id == id)
    }

    pub fn update_recipe(&mut self, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> bool{
        //to implement
    }
}