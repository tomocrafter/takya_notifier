-- Your SQL goes here
-- kind and exterior will be null if item is Vanilla
CREATE TABLE item(
    order_id int not null primary key,
    name varchar(36) not null,
    kind varchar(36) null,
    exterior enum('BS', 'WW', 'FT', 'MW', 'FN') null,
    price int not null,
    has_sold tinyint(1) default 0 not null,
    is_stattrak tinyint(1) default 0 not null
)