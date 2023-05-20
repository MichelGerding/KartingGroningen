SET statement_timeout = 0;
SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;
SET default_tablespace = '';
SET default_table_access_method = heap;

CREATE TABLE public.drivers (
                                id integer NOT NULL,
                                name character varying NOT NULL,
                                rating double precision DEFAULT 25 NOT NULL,
                                uncertainty double precision DEFAULT ((25)::numeric / 3.0) NOT NULL
);

CREATE TABLE public.heats (
                              id integer NOT NULL,
                              heat_id character varying NOT NULL,
                              heat_type character varying NOT NULL,
                              start_date timestamp without time zone NOT NULL
);


CREATE TABLE public.karts (
                              id integer NOT NULL,
                              number integer NOT NULL,
                              is_child_kart boolean DEFAULT false NOT NULL
);


CREATE TABLE public.laps (
                             id integer NOT NULL,
                             heat integer NOT NULL,
                             driver integer NOT NULL,
                             lap_in_heat integer NOT NULL,
                             lap_time double precision NOT NULL,
                             kart_id integer NOT NULL
);


ALTER TABLE public.laps OWNER TO postgres;
ALTER TABLE public.laps ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.laps_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
    );
ALTER TABLE ONLY public.drivers
    ADD CONSTRAINT drivers_name_key UNIQUE (name);
ALTER TABLE ONLY public.drivers
    ADD CONSTRAINT drivers_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.session
    ADD CONSTRAINT heats_heat_id_key UNIQUE (heat_id);
ALTER TABLE ONLY public.session
    ADD CONSTRAINT heats_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.cars
    ADD CONSTRAINT karts_number_key UNIQUE (number);
ALTER TABLE ONLY public.cars
    ADD CONSTRAINT karts_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.laps
    ADD CONSTRAINT laps_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.laps
    ADD CONSTRAINT fk_laps_driver FOREIGN KEY (driver) REFERENCES public.drivers(id);
ALTER TABLE ONLY public.laps
    ADD CONSTRAINT fk_laps_heat FOREIGN KEY (heat) REFERENCES public.session(id);
ALTER TABLE ONLY public.laps
    ADD CONSTRAINT fk_laps_kart FOREIGN KEY (kart_id) REFERENCES public.cars(id);

ALTER TABLE public.drivers OWNER TO postgres;
ALTER TABLE public.drivers ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.drivers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
    );
ALTER TABLE public.session OWNER TO postgres;
ALTER TABLE public.session ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.heats_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
    );
ALTER TABLE public.cars OWNER TO postgres;
ALTER TABLE public.cars ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.karts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
    );
