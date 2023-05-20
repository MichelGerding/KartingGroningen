

--: Lap()

--! get_lap : Lap
select * from public.laps where id = :id;

--! get_laps_from_driver : Lap
select * from public.laps where driver = :driver;

--! get_laps_from_kart : Lap
select * from public.laps where kart_id = :kart;

--! get_laps_from_drivers: Lap
select * from public.laps where driver = any(:driver::int[]);

--! get_laps_from_heat : Lap
select * from public.laps where heat = :heat;

--! get_laps_from_heats : Lap
select * from public.laps where heat = any(:heats::int[]);


--! delete_laps_from_heat
delete from public.laps where heat = :heat;


--! insert_laps_bulk: Lap
INSERT INTO
    public.laps (heat, driver, lap_in_heat, lap_time, kart_id)
select
    unnest(:heats::int[]),
    unnest(:drivers::int[]),
    unnest(:laps_in_heat::int[]),
    unnest(:lap_times::float[]),
    unnest(:karts::int[])
returning *;


--! insert_lap: Lap
INSERT INTO
    public.laps (heat, driver, lap_in_heat, lap_time, kart_id)
VALUES (:heat, :driver, :lap_in_heat, :laptime, :kart_id)
returning *;

