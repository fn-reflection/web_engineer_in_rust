CREATE TABLE IF NOT EXISTS iris_measurements (
  id SERIAL,
  sepal_length DOUBLE NOT NULL,
  sepal_width DOUBLE NOT NULL,
  petal_length DOUBLE NOT NULL,
  petal_width DOUBLE NOT NULL,
  class VARCHAR(16) NOT NULL
);