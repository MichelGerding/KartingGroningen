use barrel::{types, Migration, backend::Pg};


pub fn migration() -> String {
    let mut m = Migration::new();
    m.change_table("cars", |t| {
        t.add_column("brand", types::varchar(32).default("RiM0"));
        t.add_column("model", types::varchar(32).default("ALPHA2"));
        t.add_column("horsepower", types::integer().default(9));
        t.add_column("modified", types::boolean().default(false));
        t.drop_column("is_child_kart")
    });

    m.inject_custom("ALTER TABLE cars ALTER COLUMN brand DROP DEFAULT;");
    m.inject_custom("ALTER TABLE cars ALTER COLUMN model DROP DEFAULT;");
    m.inject_custom("ALTER TABLE cars ALTER COLUMN horsepower DROP DEFAULT;");
    m.inject_custom("ALTER TABLE cars ALTER COLUMN modified DROP DEFAULT;");

    m.make::<Pg>()
}