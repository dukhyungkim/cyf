CREATE TABLE public.images
(
    id serial NOT NULL,
    title text NOT NULL,
    url text NOT NULL,
    alt_text text,
    PRIMARY KEY (id)
);
