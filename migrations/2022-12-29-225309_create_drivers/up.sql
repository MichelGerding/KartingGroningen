create table public.Drivers
(
    "id"            INT GENERATED ALWAYS AS IDENTITY,
    "name"          varchar not null unique ,
    "fastest_lap"    integer,

    PRIMARY KEY ("id")
);
