echo "Add item: "
result=$(curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"name":"myitem", "price":125.1}' \
	127.0.0.1:8080/add_item)
echo "result: "$result
data='{"id":'$result'}'
echo "Data: "$data
echo " "
echo "Get item: "
curl --header "Content-Type: application/json" \
	--request GET \
	--data $data \
	127.0.0.1:8080/item
echo " "
echo "Create Agent: "
curl -X POST 127.0.0.1:8080/create_agent
echo " "
echo "Get agents: "
curl 127.0.0.1:8080/get_agents
echo " "
