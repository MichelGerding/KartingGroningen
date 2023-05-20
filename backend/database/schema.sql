CREATE TABLE public.cars (
                             id integer NOT NULL,
                             number integer NOT NULL,
                             brand character varying NOT NULL,
                             model character varying NOT NULL,
                             horsepower integer NOT NULL,
                             modified boolean NOT NULL
);
CREATE TABLE public.session (
                                id integer NOT NULL,
                                heat_id character varying NOT NULL,
                                heat_type character varying NOT NULL,
                                start_date timestamp without time zone NOT NULL
);
CREATE TABLE public.drivers (
                                id integer NOT NULL,
                                name character varying NOT NULL,
                                rating double precision NOT NULL,
                                uncertainty double precision NOT NULL
);
CREATE TABLE public.laps (
                             id integer NOT NULL,
                             heat integer NOT NULL,
                             driver integer NOT NULL,
                             lap_in_heat integer NOT NULL,
                             lap_time double precision NOT NULL,
                             kart_id integer NOT NULL
);
CREATE TABLE public.refinery_schema_history (
                                                version integer NOT NULL,
                                                name character varying NULL,
                                                applied_on character varying NULL,
                                                checksum character varying NULL
);
CREATE UNIQUE INDEX karts_pkey ON public.cars USING btree (id);
CREATE UNIQUE INDEX heats_pkey ON public.session USING btree (id);
CREATE UNIQUE INDEX karts_number_key ON public.cars USING btree (number);
CREATE UNIQUE INDEX drivers_pkey ON public.drivers USING btree (id);
CREATE UNIQUE INDEX drivers_name_key ON public.drivers USING btree (name);
CREATE UNIQUE INDEX heats_heat_id_key ON public.session USING btree (heat_id);
CREATE UNIQUE INDEX refinery_schema_history_pkey ON public.refinery_schema_history USING btree (version);
CREATE UNIQUE INDEX laps_pkey ON public.laps USING btree (id);
ALTER TABLE session ADD CONSTRAINT heats_heat_id_key_ UNIQUE (heat_id);
ALTER TABLE cars ADD CONSTRAINT karts_number_key_ UNIQUE (number);
ALTER TABLE laps ADD FOREIGN KEY (kart_id) REFERENCES cars(id);
ALTER TABLE laps ADD FOREIGN KEY (driver) REFERENCES drivers(id);
ALTER TABLE drivers ADD CONSTRAINT drivers_name_key_ UNIQUE (name);
ALTER TABLE laps ADD PRIMARY KEY (id);
ALTER TABLE session ADD PRIMARY KEY (id);
ALTER TABLE drivers ADD PRIMARY KEY (id);
ALTER TABLE refinery_schema_history ADD PRIMARY KEY (version);
ALTER TABLE laps ADD FOREIGN KEY (heat) REFERENCES session(id);
ALTER TABLE cars ADD PRIMARY KEY (id);
