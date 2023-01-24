create table public.Laps
(
    "id"            INT GENERATED ALWAYS AS IDENTITY,
    "heat"          INT NOT NULL,
    "driver"        INT NOT NULL,
    "lap_in_heat"     INT NOT NULL,
    "lap_time"       FLOAT NOT NULL,

    PRIMARY KEY ("id"),

    CONSTRAINT fk_laps_heat
        FOREIGN KEY ("heat")
        REFERENCES public.Heats("id"),
    CONSTRAINT fk_laps_driver
        FOREIGN KEY ("driver")
        REFERENCES public.Drivers("id")
);
