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

INSERT INTO public.users (id, email, phone) VALUES (1, 'hershel@example.com', '(701) 507-6868');
INSERT INTO public.users (id, email, phone) VALUES (2, 'edwardo@example.net', '923-201-3231 x5145');
INSERT INTO public.users (id, email, phone) VALUES (3, 'hershel@example.com', '1-412-082-0525 x467');