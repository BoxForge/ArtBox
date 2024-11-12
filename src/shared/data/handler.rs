#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use directories::ProjectDirs;
use std::path::Path;
use std::sync::Arc;
use rusqlite::{params, Connection, Result};

struct Hercules {
    images: Connection,
    palettes: Connection,
}

impl Hercules {
    fn new(images_db_path: &str, palettes_db_path: &str) -> Result<Hercules> {
        let images_db = Connection::open("images.db")?;
        let palettes_db = Connection::open("palettes.db")?;
        Ok(Hercules {
            images: images_db,
            palettes: palettes_db,
        })
    }

// TODO: Create Sprite & Palette Databases
// TODO: Add CRUD Functions

}

struct Hermes {
    handler: Arc<Hercules>,
}

impl Hermes {
    fn new(handler: Arc<Hercules>) -> Self {
        Hermes { handler }
    }
    // TODO: Add CRUD Functions
}
