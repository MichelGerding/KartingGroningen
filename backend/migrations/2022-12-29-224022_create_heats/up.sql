create table public.Heats
(
    "id" int GENERATED ALWAYS AS IDENTITY,
    "heat_id"      varchar not null UNIQUE ,
    "heat_type"    varchar not null,
    "start_date"   timestamp not null,

    PRIMARY KEY ("id")
);
