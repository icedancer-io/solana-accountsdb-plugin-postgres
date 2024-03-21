use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    // Establish a connection to the default database for the user
    let mut client = Client::connect(
        "host=localhost user=postgres password=password dbname=solana",
        NoTls,
    )?;

    let query = client.query("SELECT * FROM transaction", &[])?;
    println!("{:?}", query);

    // Perform database operations
    // For example, executing a query
    // for row in client.query("SELECT id, name FROM your_table", &[])? {
    //     let id: i32 = row.get(0);
    //     let name: &str = row.get(1);
    //     println!("id: {}, name: {}", id, name);
    // }

    Ok(())
}
