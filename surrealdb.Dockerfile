FROM surrealdb/surrealdb:latest
EXPOSE 8000
CMD ["start", "--bind", "0.0.0.0:8080", "file://data/srdb.db"]