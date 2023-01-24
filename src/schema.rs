// @generated automatically by Diesel CLI.

diesel::table! {
    drivers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    heats (id) {
        id -> Int4,
        heat_id -> Varchar,
        heat_type -> Varchar,
        start_date -> Timestamp,
    }
}

diesel::table! {
    karts (id) {
        id -> Int4,
        number -> Int4,
        is_child_kart -> Bool,
    }
}

diesel::table! {
    laps (id) {
        id -> Int4,
        heat -> Int4,
        driver -> Int4,
        lap_in_heat -> Int4,
        lap_time -> Float8,
        kart_id -> Int4,
    }
}

diesel::joinable!(laps -> heats (heat));
diesel::joinable!(laps -> karts (kart_id));

diesel::allow_tables_to_appear_in_same_query!(drivers, heats, karts, laps,);
