-- Sample SQL dump with PII
CREATE TABLE users (
    id INT PRIMARY KEY,
    email VARCHAR(255),
    phone VARCHAR(20)
);

INSERT INTO users (id, email, phone) VALUES (1, 'john.doe@example.com', '555-123-4567');
INSERT INTO users (id, email, phone) VALUES (2, 'jane.smith@test.com', '555-987-6543');
INSERT INTO users (id, email, phone) VALUES (3, 'john.doe@example.com', '555-555-5555');
