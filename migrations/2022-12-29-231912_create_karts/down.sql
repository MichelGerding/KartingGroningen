ALTER TABLE Laps
    DROP COLUMN IF EXISTS "kart_id";

alter table Laps
    drop constraint if exists fk_laps_kart;

DROP TABLE IF EXISTS Karts;