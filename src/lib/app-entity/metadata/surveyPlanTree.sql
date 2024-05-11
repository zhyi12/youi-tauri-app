
create table  stats_survey_plan_tree
(
    id    INTEGER      not null
        primary key autoincrement,

    pid    INTEGER ,
    num    TEXT ,
    text    TEXT ,
    object_name    TEXT ,
    object_id    INTEGER ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);