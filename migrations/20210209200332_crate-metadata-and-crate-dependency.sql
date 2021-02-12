create table crate_metadata
(
    id           serial      not null,
    name         varchar(64) not null,
    version      varchar(40) not null,
    dependencies integer     not null,
    constraint crate_metadata_pk
        primary key (id)
);

create unique index crate_metadata_name_version_uindex
    on crate_metadata (name, version);

create table crate_dependency
(
    id          serial      not null,
    crate_id    integer     not null,
    name        varchar(64) not null,
    requirement varchar(40) not null,
    type        varchar(6)  not null,
    constraint crate_dependency_pk
        primary key (id),
    constraint crate_dependency_crate_metadata_id_fk
        foreign key (crate_id) references crate_metadata
            on delete cascade
);

create unique index crate_dependency_name_type_crate_id_uindex
    on crate_dependency (name, type, crate_id);
