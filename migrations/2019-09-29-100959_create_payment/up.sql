create type payment_state AS ENUM ('open', 'paid', 'complete');
create type payment_environment AS ENUM ('test', 'live');

create table payments (
    id uuid not null primary key,
    time timestamp not null default now(),
    state payment_state not null default 'open',
    customer_id uuid not null,
    environment payment_environment not null default 'live',
    payment_method varchar
);

create table payment_items (
    id uuid not null primary key,
    payment_id uuid not null references payments(id),
    item_type varchar not null,
    item_data jsonb not null,
    title varchar not null,
    quantity int not null,
    price money not null
);