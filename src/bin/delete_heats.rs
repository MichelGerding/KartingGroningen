use dotenvy::dotenv;

use karting_groningen_analytics::modules::models::general::establish_connection;
use karting_groningen_analytics::modules::models::heat::Heat;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let connection = &mut establish_connection();

    let heats = [
        "0222A7E031E248678053CD350479FD87",
        "0BB56036B7684EAE9158F9362A14036F",
        "0C69F53C49A3479FA9035D8EA62D8133",
        "0E8A7B5E6AB147298D0E1FAF7056F2E1",
        "1374B2643F9C4C6BB2A3491DCE16C649",
        "20D4A978A65B4BD8A3EF1139377F4350",
        "36979906586545CD9B90F6BA8E9C50C2",
        "37597A3FCE074FEA9A6DB35294B31AF2",
        "3CF4B99FD5AF4F9DB0D7A4BD6034E969",
        "5AFC03B1849A416A86CD7FA71F3BEC27",
        "67C25AB5A8E44CD09216BCAEBCB7442C",
        "6D57BA5EBD3C40DDACE24D4C25868BF0",
        "8BA6BB8672B546D3B1892569FB7288AD",
        "9E0E3B135CBC46D28D9419A2B461F85A",
        "C450A88F5892441A875C9D948011A384",
        "C9855916612B41FB98C786F86059C866",
        "CC0E316EA2DE45C59D5D81A67E671F9F",
        "D184F4AC03AB4CD5A642573C0404DD4C",
        "D700E52814D64DFAAA73FEFD899D2275",
        "FBEEF1C69AC143A69AA80ACE7895A526",
    ];

    for heat_id in heats {
        let heat = Heat::delete_id(connection, heat_id);
        println!("Deleted heat: {:?}", heat);
    }
}
