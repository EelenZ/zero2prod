


docker run --name postgres-0  -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -p 5432:5432 -d postgres postgres -N 1000



set DATABASE_URL=postgres://postgres:password@localhost:5432/$newsletter

