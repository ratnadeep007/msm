use sqlite::Connection;
pub fn create_tables(connection: &Connection) {
    connection
        .execute(
            "
            CREATE TABLE runners (
                id INTEGER PRIMARY KEY,
                run_type TEXT,
                exec_path TEXT,
                entry_point TEXT
            );

            CREATE TABLE services (
                id INTEGER PRIMARY KEY,
                name TEXT, 
                availible BOOL,
                path TEXT,
                envs TEXT,
                runner INT,
                FOREIGN KEY (runner) REFERENCES runners(id)
            );
        ",
        )
        .unwrap();
}
