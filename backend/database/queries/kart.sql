--
-- CLASSES
--

--: Kart()
--: KartWithStats()


--
-- GETTERS
--

--! get_all_karts : Kart
select * from public.cars;
--
--! get_kart_by_id : Kart
select * from public.cars WHERE id = :id;
--
--! get_karts_by_ids : Kart
select * from public.cars WHERE id = any(:ids::int[]);

--! get_kart_by_number : Kart
select * from public.cars WHERE number = :number;

--! get_karts_by_numbers : Kart
select * from public.cars WHERE number = any(:numbers::int[]);



--! get_kart_from_lap : Kart
SELECT k.* FROM public.cars k
    INNER JOIN public.laps l on k.id = l.kart_id
WHERE l.id = :lap;



--! get_karts_from_laps : Kart
SELECT k.* FROM public.cars k
    INNER JOIN public.laps l on k.id = l.kart_id
WHERE l.id = any(:laps::int[]);

--! get_karts_stats_per_day
select
    k.*,
    h.start_date,
    min(lap_time) as min_laptime,
    avg(lap_time) as avg_laptime,
    percentile_cont(0.5) WITHIN GROUP (ORDER BY lap_time) as median_laptime
from public.cars k
         inner join public.laps l on k.id = l.kart_id
         inner join public.session h on h.id = l.heat
group by k.id, k.number, h.start_date;


--! get_kart_with_stats : KartWithStats
select
    k.*,
    CAST(count(l.id) AS INT) as lap_count,
    CAST(count(DISTINCT l.driver) AS INT) as driver_count
from public.cars k
inner join public.laps l on k.id = l.kart_id
where k.number = :kart_number
group by k.id;

--! get_all_karts_with_stats : KartWithStats
select
    k.*,
    CAST(count(l.id) AS INT) as lap_count,
    CAST(count(DISTINCT l.driver) AS INT) as driver_count
from public.cars k
         inner join public.laps l on k.id = l.kart_id
group by k.id
order by :order_column;

--! create_kart
INSERT INTO public.cars (number, brand, model, horsepower, modified) values (:number, :brand, :model, :horsepower, :modified) RETURNING id;
