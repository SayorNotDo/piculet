--: User()

--! get_users : User
SELECT id,
       email
FROM users;

--! insert_user
INSERT INTO users(uuid, username, hashed_password)
VALUES (:uuid, :username, :hashed_password);