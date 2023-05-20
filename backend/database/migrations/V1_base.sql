CREATE TABLE public.laps (
                             id integer NOT NULL,
                             lap_in_heat integer NOT NULL,
                             kart_id integer NOT NULL,
                             heat integer NOT NULL,
                             driver integer NOT NULL,
                             lap_time double precision NOT NULL
);
CREATE TABLE public.heats (
                              start_date timestamp without time zone NOT NULL,
                              heat_id character varying NOT NULL,
                              id integer NOT NULL,
                              heat_type character varying NOT NULL
);
CREATE TABLE public.drivers (
                                id integer NOT NULL,
                                name character varying NOT NULL,
                                uncertainty double precision NOT NULL,
                                rating double precision NOT NULL
);
CREATE TABLE public.karts (
                              number integer NOT NULL,
                              is_child_kart boolean NOT NULL,
                              name character varying(255) NULL,
                              id integer NOT NULL
);
CREATE UNIQUE INDEX heats_pkey ON public.session USING btree (id);
CREATE UNIQUE INDEX drivers_name_key ON public.drivers USING btree (name);
CREATE UNIQUE INDEX karts_number_key ON public.cars USING btree (number);
CREATE UNIQUE INDEX drivers_pkey ON public.drivers USING btree (id);
CREATE UNIQUE INDEX heats_heat_id_key ON public.session USING btree (heat_id);
CREATE UNIQUE INDEX karts_pkey ON public.cars USING btree (id);
CREATE UNIQUE INDEX laps_pkey ON public.laps USING btree (id);
ALTER TABLE laps ADD FOREIGN KEY (heat) REFERENCES session(id);
ALTER TABLE drivers ADD CONSTRAINT drivers_name_key_ UNIQUE (name);
ALTER TABLE laps ADD FOREIGN KEY (driver) REFERENCES drivers(id);
ALTER TABLE drivers ADD PRIMARY KEY (id);
ALTER TABLE laps ADD FOREIGN KEY (kart_id) REFERENCES cars(id);
ALTER TABLE laps ADD PRIMARY KEY (id);
ALTER TABLE cars ADD CONSTRAINT karts_number_key_ UNIQUE (number);
ALTER TABLE cars ADD PRIMARY KEY (id);
ALTER TABLE session ADD CONSTRAINT heats_heat_id_key_ UNIQUE (heat_id);
ALTER TABLE session ADD PRIMARY KEY (id);
