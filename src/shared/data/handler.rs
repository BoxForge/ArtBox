#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use directories::ProjectDirs;
use std::path::Path;
use std::sync::Arc;
use rusqlite::{params, Connection, Result};

struct Hercules {
    assets: Connection,
}

impl Hercules {
    fn new() -> Result<Hercules> {
        let assets_db_path = Self::get_db_path("assets.db");

        let assets_db = Connection::open(assets_db_path)?;

        let mut hercules = Hercules { assets: assets_db };
        hercules.setup_database()?;
         
        Ok(hercules)
    }

    fn get_db_path(db_name: &str) -> String {
        let base_path = if cfg!(debug_assertions) {
            "./".to_string()
        } else {
            directories::ProjectDirs::from("com", "BoxForge", "ArtBox")
                .map(|proj_dirs| proj_dirs.data_local_dir().to_str().unwrap_or(".").to_string())
                .unwrap_or_else(|| ".".to_string())
        };

        format!("{}/{}", base_path, db_name)
    }

    fn setup_database(&mut self) -> Result<(), rusqlite::Error> {
        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS images (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                image_data BLOB NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT,
                tags TEXT,
                palette_id INTEGER,
                FOREIGN KEY(palette_id) REFERENCES palettes(id)
            );",
            [],
        )?;

        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS palettes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT,
                tags TEXT
            );",
            [],
        )?;

        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS colors (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                hex VARCHAR(7) NOT NULL,
                rgb_r INTEGER NOT NULL,
                rgb_g INTEGER NOT NULL,
                rgb_b INTEGER NOT NULL,
                hsv_h INTEGER NOT NULL,
                hsv_s INTEGER NOT NULL,
                hsv_v INTEGER NOT NULL
            );",
            [],
        )?;

        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS palettes_colors (
                palette_id INTEGER NOT NULL,
                color_id INTEGER NOT NULL,
                PRIMARY KEY (palette_id, color_id),
                FOREIGN KEY(palette_id) REFERENCES palettes(id),
                FOREIGN KEY(color_id) REFERENCES colors(id)
            );",
            [],
        )?;

        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT,
                num_attached INTEGER DEFAULT 0
            );",
            [],
        )?;

        self.assets.execute(
            "CREATE TABLE IF NOT EXISTS item_tags (
                item_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                item_type TEXT NOT NULL,
                PRIMARY KEY (item_id, tag_id),
                FOREIGN KEY(tag_id) REFERENCES tags(id)
            );",
            [],
        )?;

        Ok(())
    }

}

struct Hermes {
    hercules: Hercules,
}

impl Hermes {
    fn new(handler: Arc<Hercules>) -> Self {
        let hercules = Hercules::new();
        Hermes { hercules: hercules.unwrap() }
    }
    // TODO: Add CRUD Functions
}
