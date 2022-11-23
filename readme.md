# rust server 

## setup database

### load docker 
```
docker-compose up -d
```
### load mock data
```
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql
```

## run server 
```
cargo run
```

### test
```
curl http://localhost:8080/ | jq .
curl http://localhost:8080/todos | jq .
```

