CREATE SCHEMA "hcs_changes";

CREATE TABLE "hcs_changes"."files" ("file_id" SERIAL PRIMARY KEY);

CREATE TABLE "hcs_changes"."change_types" (
    "change_type_id" SMALLINT NOT NULL PRIMARY KEY,
    "name" VARCHAR(32) NOT NULL
);

-- `hcs_changes`.`change_types` table is populated with `5 <= change_type_id <= 16`
-- The following `id`s relate to the `data_uid` of each default `Data` object.
INSERT INTO
    "hcs_changes"."change_types" ("change_type_id", "name")
VALUES
    -- path of new file
    (5, 'file_create'),
    -- no path, just an event
    (6, 'file_modify'),
    -- new path of file
    (7, 'file_move'),
    -- path of trashed location (TODO future update)
    (8, 'file_delete'),
    -- original path of file (TODO future update)
    (9, 'file_undo_delete'),
    -- path of symlink, split by `;` into `path;points_to`
    (10, 'symlink_create'),
    -- no path, just an event
    (11, 'symlink_delete'),
    -- path of new directory
    (12, 'directory_create'),
    -- new path of directory
    (13, 'directory_move'),
    -- path of trashed location (TODO future update)
    (14, 'directory_delete'),
    -- original path of directory (TODO future update)
    (15, 'directory_undo_delete');

CREATE TABLE "hcs_changes"."file_changes" (
    "change_id" SERIAL PRIMARY KEY,
    "file_id" INT NOT NULL,
    "time" TIMESTAMP NOT NULL,
    "path" TEXT,
    "change_type_id" SMALLINT NOT NULL,
    FOREIGN KEY ("file_id") REFERENCES "hcs_changes"."files"("file_id"),
    FOREIGN KEY ("change_type_id") REFERENCES "hcs_changes"."change_types"("change_type_id")
);

ALTER TABLE
    "hcs_changes"."file_changes"
ADD
    CONSTRAINT "change_types_path_not_null" CHECK (
        (
            "change_type_id" IN (5, 7, 8, 9, 10, 12, 13, 14, 15)
            AND "path" IS NOT NULL
        )
        OR (
            "change_type_id" IN (6, 11)
            AND "path" IS NULL
        )
    );

CREATE UNIQUE INDEX "unique_change_types_path_not_null" ON "hcs_changes"."file_changes" ("change_type_id", "path")
WHERE
    (
        (
            "change_type_id" IN (5, 7, 8, 9, 10, 12, 13, 14, 15)
            AND "path" IS NOT NULL
        )
        OR (
            "change_type_id" IN (6, 11)
            AND "path" IS NULL
        )
    );

CREATE
OR REPLACE FUNCTION set_file_changes_time() RETURNS TRIGGER AS $ $ BEGIN NEW.time = NOW();

RETURN NEW;

END;

$ $ LANGUAGE plpgsql;

CREATE TRIGGER file_changes_set_time BEFORE
INSERT
    ON "hcs_changes"."file_changes" FOR EACH ROW EXECUTE FUNCTION set_file_changes_time();