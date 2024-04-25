
create table  youi_desktop_item
(
    id    INTEGER      not null
        primary key autoincrement,

    pid    INTEGER ,
    num    TEXT ,
    text    TEXT ,
    item_type    TEXT ,
    uri    TEXT ,
    image    TEXT ,
    icon    TEXT ,
    params    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT ,
);