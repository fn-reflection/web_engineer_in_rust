ARG MYSQL_VERSION
FROM mysql:${MYSQL_VERSION}
COPY ./docker/mysql/build/etc/mysql/conf.d/my.cnf /etc/mysql/conf.d/my.cnf
COPY ./docker/mysql/build/docker-entrypoint-initdb.d/initialize.sql /docker-entrypoint-initdb.d/initialize.sql
RUN mkdir -p /var/lib/mysql-files