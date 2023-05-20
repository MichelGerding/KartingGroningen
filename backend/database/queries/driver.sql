--
-- CLASSES
--

--: Driver()


--
-- GETTERS
--

--! get_all_drivers : Driver
select * from public.drivers;

--! get_driver_by_id : Driver
select * from public.drivers WHERE id = :id;

--! get_drivers_by_ids : Driver
select * from public.drivers WHERE id = any(:ids::int[]);

--: DriverWithStats()
--! search_driver_with_stats_paginated : DriverWithStats
select
    d.*,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as avg_lap_time,
    percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
    CAST(count(l.lap_time) AS INT) as total_laps,
    CAST(count(DISTINCT l.heat) AS INT) as total_heats
from public.drivers d
         inner join public.laps l on d.id = l.driver
where d.name like concat('%', :name::text ,'%')
GROUP BY d.id
limit :limit offset :offset;

--! get_driver_with_stats : DriverWithStats
select
    d.*,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as avg_lap_time,
    percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
    CAST(count(l.lap_time) AS INT) as total_laps,
    CAST(count(DISTINCT l.heat) AS INT) as total_heats
from public.drivers d
         inner join public.laps l on d.id = l.driver
where d.id = :id::int
GROUP BY d.id;

--! get_driver_by_name : Driver
select
    *
from public.drivers
WHERE name = :name::text;

--! search_driver_by_name : Driver
select
    *
from public.drivers
WHERE name LIKE concat('%', :name::text, '%')
LIMIT :limit OFFSET :offset;

--! get_drivers_from_heat_in_order_fastest_lap
select
    d.*
from public.drivers d
inner join public.laps l on d.id = l.driver
inner join public.session h on h.id = l.heat
where h.id = :heat
group by d.id
order by min(l.lap_time) asc;


--! update_driver_rating
with update_rows AS (
    UPDATE
        public.drivers
    set
        rating=:rating,
        uncertainty=:uncertainty
    where id = :id
    RETURNING 1
) SELECT count(*) FROM update_rows;

--! create_driver : Driver
INSERT INTO public.drivers (name, rating, uncertainty) values (:name, :rating, :uncertainty) RETURNING *;
