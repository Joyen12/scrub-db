-- PostgreSQL database dump
-- Dumped from database version 14.5
-- Dumped by pg_dump version 14.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';

CREATE TABLE public.users (
    id integer NOT NULL,
    email character varying(255),
    phone character varying(20),
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;
ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);

INSERT INTO public.users (id, email, phone) VALUES (1, 'leo@example.org', '1-902-104-3259 x756');
INSERT INTO public.users (id, email, phone) VALUES (2, 'doyle@example.org', '669.004.2719 x98304');
INSERT INTO public.users (id, email, phone) VALUES (3, 'leo@example.org', '246.873.3786 x1917');
