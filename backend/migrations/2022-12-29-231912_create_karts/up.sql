create table public.Karts
(
    "id"            INT GENERATED ALWAYS AS IDENTITY,
    "number"        int not null UNIQUE ,
    "is_child_kart"   boolean not null DEFAULT false,

    PRIMARY KEY ("id")
);

ALTER TABLE Laps
    ADD COLUMN "kart_id" INT;

alter table Laps
    add constraint fk_laps_kart
        foreign key ("kart_id") references Karts ("id");

