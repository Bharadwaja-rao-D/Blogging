-- Your SQL goes here

create table student(
	id INTEGER PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	password TEXT NOT NULL
);

create table content(
	id INTEGER PRIMARY KEY NOT NULL,
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	body TEXT NOT NULL,
	creator_id INTEGER NOT NULL,
	upvotes	INTEGER default 0 NOT NULL,
	FOREIGN KEY(creator_id) REFERENCES student(id) ON DELETE CASCADE
);

create table commenting(
	id INTEGER PRIMARY KEY NOT NULL,
	commentor_id INTEGER NOT NULL,
	content_id INTEGER NOT NULL,
	FOREIGN KEY(commentor_id) REFERENCES student(id) ON DELETE CASCADE
	FOREIGN KEY(content_id) REFERENCES content(id) ON DELETE CASCADE
);
