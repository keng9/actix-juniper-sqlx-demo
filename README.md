# Actix Juniper Sqlx Sample
Query
````graphql
{
    GetHuman(name: ""){id}
}
````
Mutation
````graphql
mutation{createHumanData(name:""){
    id}
}
````