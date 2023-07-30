use rusqlite::{params, Connection, Result};

#[derive(Debug)]
#[allow(unused)]
struct Person {
    id: u64,
    name: String,
    data: Option<Vec<u8>>,
}

fn create_db() -> Result<Connection> {
    let database_file = "data.db";
    let conn = Connection::open(database_file)?;
    let _ = conn.execute("DROP TABLE person", []);

    conn.execute(
        "CREATE TABLE person (
            id  INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BLOB    
        );",
        [],
    )?;

    Ok(conn)
}

fn insert_data(conn: &Connection) -> Result<()> {
    let p1 = Person {
        id: 1,
        name: "Dave".to_string(),
        data: None,
    };

    let p2 = Person {
        id: 2,
        name: "Nick".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (id, name, data)
        VALUES (?1, ?2, ?3), (?4, ?5, ?6);",
        params![p1.id, p1.name, p1.data, p2.id, p2.name, p2.data],
    )?;

    Ok(())
}

fn get_data(conn: &Connection) -> Result<Vec<Person>> {
    let mut stmt = conn.prepare("SELECT id, name, data FROM person;")?;
    let persons_iterator = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;
    let mut persons = vec![];
    for p in persons_iterator {
        persons.push(p?);
    }
    Ok(persons)
}

fn main() -> Result<()> {
    let conn = create_db()?;
    insert_data(&conn)?;
    let persons = get_data(&conn)?;
    for p in persons {
        println!("Hi: {:?}", p);
    }
    Ok(())
}
