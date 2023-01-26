use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::models::heat::Heat;

pub fn main() {
    let connection = &mut establish_connection();

    let heats = Heat::get_all_chronologicaly(connection);
    for heat in heats {
        println!("applying ratings of heat: {} ", heat.heat_id);
        heat.apply_ratings(connection);
    }
}