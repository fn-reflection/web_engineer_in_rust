CREATE TABLE IF NOT EXISTS iris_measurements (
  id SERIAL,
  sepal_length DOUBLE NOT NULL, -- がくの長さ
  sepal_width DOUBLE NOT NULL, -- がくの幅
  petal_length DOUBLE NOT NULL, -- 花弁の長さ
  petal_width DOUBLE NOT NULL, -- 花弁の幅
  class VARCHAR(16) NOT NULL -- 分類名
);
