pub const CREATE_LIST_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS list (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
";

pub const CREATE_WORD_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS word (
        id INTEGER PRIMARY KEY,
        list_id INTEGER NOT NULL,
        word TEXT NOT NULL,
        reading TEXT NOT NULL,
        translation TEXT NOT NULL,
        frequency INTEGER NOT NULL,
        FOREIGN KEY(list_id) REFERENCES list(id)
    );
";
