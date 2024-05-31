
create table  youi_area
(
    id    INTEGER      not null
        primary key ,

    pid    TEXT ,
    num    TEXT ,
    text    TEXT ,
    full_text    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);