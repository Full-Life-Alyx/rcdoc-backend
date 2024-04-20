CREATE TABLE "permission"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "created_by" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL
);
ALTER TABLE
    "permission" ADD PRIMARY KEY("id");
CREATE TABLE "permission_assignment"(
    "id" SERIAL NOT NULL,
    "permission_id" INTEGER NOT NULL,
    "user_id" UUID NOT NULL,
    "assigner_id" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "operation" VARCHAR(255)
    CHECK
        ("operation" IN('')) NOT NULL
);
ALTER TABLE
    "permission_assignment" ADD PRIMARY KEY("id");
CREATE TABLE "tag"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "category_id" INTEGER NOT NULL,
    "created_by" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "year" SMALLINT NULL
);
ALTER TABLE
    "tag" ADD PRIMARY KEY("id");
ALTER TABLE
    "tag" ADD CONSTRAINT "tag_name_unique" UNIQUE("name");
CREATE TABLE "tag_category"(
    "id" SERIAL NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "created_by" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL
);
ALTER TABLE
    "tag_category" ADD PRIMARY KEY("id");
ALTER TABLE
    "tag_category" ADD CONSTRAINT "tag_category_name_unique" UNIQUE("name");
CREATE TABLE "revision"(
    "id" UUID NOT NULL,
    "version" INTEGER NOT NULL,
    "document_id" UUID NOT NULL,
    "author" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL
);
ALTER TABLE
    "revision" ADD PRIMARY KEY("id");
CREATE TABLE "document"(
    "id" UUID NOT NULL,
    "timestamp" TIMESTAMP(0) WITH
        TIME zone NOT NULL,
        "identifier" INTEGER NOT NULL,
        "checked_out_by" UUID NOT NULL,
        "comment_of" UUID NOT NULL
);
ALTER TABLE
    "document" ADD PRIMARY KEY("id");
CREATE TABLE "user"(
    "id" UUID NOT NULL,
    "nickname" VARCHAR(255) NOT NULL,
    "oid" UUID NOT NULL
);
ALTER TABLE
    "user" ADD PRIMARY KEY("id");
CREATE TABLE "tag_assignment"(
    "id" bigserial NOT NULL,
    "tag_id" INTEGER NOT NULL,
    "document_id" UUID NOT NULL,
    "operation" VARCHAR(255) CHECK
        ("operation" IN('')) NOT NULL,
        "timestamp" TIMESTAMP(0)
    WITH
        TIME zone NOT NULL,
        "assigner_id" UUID NOT NULL
);
ALTER TABLE
    "tag_assignment" ADD PRIMARY KEY("id");
ALTER TABLE
    "tag" ADD CONSTRAINT "tag_created_by_foreign" FOREIGN KEY("created_by") REFERENCES "user"("id");
ALTER TABLE
    "document" ADD CONSTRAINT "document_comment_of_foreign" FOREIGN KEY("comment_of") REFERENCES "document"("id");
ALTER TABLE
    "permission" ADD CONSTRAINT "permission_created_by_foreign" FOREIGN KEY("created_by") REFERENCES "user"("id");
ALTER TABLE
    "tag_assignment" ADD CONSTRAINT "tag_assignment_tag_id_foreign" FOREIGN KEY("tag_id") REFERENCES "tag"("id");
ALTER TABLE
    "permission_assignment" ADD CONSTRAINT "permission_assignment_user_id_foreign" FOREIGN KEY("user_id") REFERENCES "user"("id");
ALTER TABLE
    "tag_assignment" ADD CONSTRAINT "tag_assignment_assigner_id_foreign" FOREIGN KEY("assigner_id") REFERENCES "user"("id");
ALTER TABLE
    "revision" ADD CONSTRAINT "revision_author_foreign" FOREIGN KEY("author") REFERENCES "user"("id");
ALTER TABLE
    "document" ADD CONSTRAINT "document_checked_out_by_foreign" FOREIGN KEY("checked_out_by") REFERENCES "user"("id");
ALTER TABLE
    "permission_assignment" ADD CONSTRAINT "permission_assignment_assigner_id_foreign" FOREIGN KEY("assigner_id") REFERENCES "user"("id");
ALTER TABLE
    "tag_assignment" ADD CONSTRAINT "tag_assignment_document_id_foreign" FOREIGN KEY("document_id") REFERENCES "document"("id");
ALTER TABLE
    "tag" ADD CONSTRAINT "tag_category_id_foreign" FOREIGN KEY("category_id") REFERENCES "tag_category"("id");
ALTER TABLE
    "revision" ADD CONSTRAINT "revision_document_id_foreign" FOREIGN KEY("document_id") REFERENCES "document"("id");
ALTER TABLE
    "permission_assignment" ADD CONSTRAINT "permission_assignment_permission_id_foreign" FOREIGN KEY("permission_id") REFERENCES "permission"("id");
ALTER TABLE
    "tag_category" ADD CONSTRAINT "tag_category_created_by_foreign" FOREIGN KEY("created_by") REFERENCES "user"("id");
