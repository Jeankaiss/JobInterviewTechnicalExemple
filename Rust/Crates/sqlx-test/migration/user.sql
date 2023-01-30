DROP TABLE IF EXISTS user;
CREATE TABLE user (
    uuid VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

INSERT INTO user 
    (uuid, first_name, last_name, phone, email, password)
VALUES
    ("001c5616-3029-4321-834b-0741871cbc6c", "Toto", "Tata", "0644958745", "test@test.fr", "test");
