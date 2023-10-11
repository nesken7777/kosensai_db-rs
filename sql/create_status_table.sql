create table status (
    id int(10) unsigned not null auto_increment primary key,
    power int(10) unsigned not null default 0,
    speed int(10) unsigned not null default 0,
    stamina int(10) unsigned not null default 0,
    luck int(10) unsigned not null default 0,
    score1 int(10) unsigned not null default 0,
    score2 int(10) unsigned not null default 0,
    name text not null
);