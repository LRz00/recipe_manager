![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
# Recipe Manager CLI

A simple command-line application to manage recipes, allowing you to add, list, update, delete, save, and load recipes from a file.

## Features

- Add new recipes with ingredients, instructions, and servings.
- List all recipes with their IDs and names.
- View a specific recipe by its ID.
- Update existing recipes.
- Delete recipes by ID.
- Save all recipes to a file.
- Load recipes from a file.

## Usage

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone https://github.com/LRz00/recipe_manager.git
   cd recipe-manager
   ```

2. Ensure you have Rust installed. If not, [install Rust](https://www.rust-lang.org/tools/install).

3. Run the application:
   ```bash
   cargo run
   ```

4. Follow the prompts in the terminal to manage your recipes.

## Menu Options

- **1**: Add Recipe  
- **2**: List Recipes  
- **3**: Find Recipe by ID  
- **4**: Update Recipe  
- **5**: Delete Recipe  
- **6**: Save Recipes to File  
- **7**: Load Recipes from File  
- **0**: Exit  

## Example Recipe Format

- **Ingredients**: tomato, cheese, basil  
- **Instructions**: chop tomato, add cheese, bake for 10 minutes  

## File Operations

- Recipes are saved and loaded from JSON files.
- Make sure the file name provided is valid when saving or loading recipes.

## Dependencies

- `serde` and `serde_json` for JSON serialization and deserialization.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Developed by Lara.
