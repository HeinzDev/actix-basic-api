#!/usr/bin/env bash

# Função para verificar se a resposta contém a string esperada
check_response() {
  if [[ $1 == *"$2"* ]]; then
    echo -e "\e[32m$3 SUCCESS\e[0m"
  else
    echo -e "\e[31m$3 FAILED\e[0m"
  fi
}

# Exemplo de uso
echo "Iniciando Teste 1"
measure_time curl -s -o /dev/null -w "%{time_total}\n" URL_1
echo "Iniciando Teste 2"
measure_time curl -s -o /dev/null -w "%{time_total}\n" URL_2

start_time=$(date +%s%3N)

echo "_______________________________________________________________________________________"

# Teste POST
echo "[POST] Should create a user"
response=$(curl -X POST -H "Content-Type: application/json" -d '{
  "username": "novo_usuario",
  "email": "novousuario@example.com"
}' http://127.0.0.1:8080/users)
check_response "$response" '{"id":1,"username":"novo_usuario","email":"novousuario@example.com"}' "[POST]"

echo "_______________________________________________________________________________________"

# Teste GET
echo "[GET] Should get all users"
response=$(curl http://127.0.0.1:8080/users)
check_response "$response" '[{"id":1,"username":"novo_usuario","email":"novousuario@example.com"}]' "[GET]"

echo "_______________________________________________________________________________________"

# Teste GET com ID (Substitua ID_DO_USUARIO pelo ID real)
echo "[GET] Should get a user by ID"
response=$(curl http://127.0.0.1:8080/users/1)
check_response "$response" '{"id":1,"username":"novo_usuario","email":"novousuario@example.com"}' "[GET]"

echo "_______________________________________________________________________________________"

# Teste PUT (Substitua SEUSERVIDOR e ID_DO_USUARIO pelos valores reais)
echo "[PUT] Should edit the user"
response=$(curl -X PUT -H "Content-Type: application/json" -d '{"username": "NovoNome", "email": "novoemail@example.com"}' http://127.0.0.1:8080/users/1)
check_response "$response" '{"id":null,"username":"NovoNome","email":"novoemail@example.com"}' "[PUT]"

echo "_______________________________________________________________________________________"

# Teste DELETE
echo "[DELETE] Should delete user by id"
response=$(curl -X DELETE http://127.0.0.1:8080/users/1)
check_response "$response" '' "[DELETE]"

echo "_______________________________________________________________________________________"

end_time=$(date +%s%3N)
elapsed_time=$((end_time - start_time))

echo "[$elapsed_time ms]"