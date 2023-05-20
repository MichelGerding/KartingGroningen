

--: Heat(id, heat_id, heat_type, start_date)
--: GetHeatWithStats()

--! get_all_heats : Heat
select * from public.session;

--! get_heat_from_id: Heat
select * from public.session where id = :id;

--! get_heats_from_ids: Heat
select * from public.session WHERE id = any(:ids::int[]);

--! get_heat_from_name: Heat
select * from public.session where heat_id = :name::text;

--! get_all_heats_with_stats : GetHeatWithStats
select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
group by h.id;

--! get_heat_with_stats_paginated : GetHeatWithStats
select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
group by h.id, start_time
limit :limit
offset :offset;

--! get_heat_with_stats : GetHeatWithStats
select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
where h.heat_id = :heat
group by h.id;

--! get_all_chronologicaly : Heat
select * from public.session order by start_date;


--! delete_heat
delete from public.session where id = :id returning *;

--! create_new_heat: Heat
insert into public.session (heat_id, heat_type, start_date) VALUES (:heat_id, :heat_type, :start_date) returning  *;