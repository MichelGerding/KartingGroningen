use barrel::{types, Migration, backend::Pg};


pub fn migration() -> String {
    let mut m = Migration::new();

    // m.creat_table("tracks", |t| {
    //     t.add_column("id", types::integer().increments(true).unique(true));
    //     t.add_column("name", types::varchar(256));
    //     t.add_column("layout", types::varchar(256));
    // });
    //
    // m.inject_custom("INSERT INTO tracks (name, layout) values ('KartbaanGroningen', 'LONG BEACH')");
    //
    // m.change_table("laps", |t| {
    //     t.add_column(
    //         "track",
    //         types::foreign(
    //             "tracks",
    //             "id",
    //             types::ReferentialAction::Cascade,
    //             types::ReferentialAction::Cascade
    //          ).default(1));
    // });


    m.make::<Pg>()
}