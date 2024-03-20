package main

import (
	"go-api/controllers"
	"go-api/middlewares"
	"go-api/pkg"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

func init() {
	err := godotenv.Load("../.env")

	if err != nil {
		panic(err)
	}

	pkg.DB = pkg.DatabaseConnect()
}

func main() {
	r := gin.Default()

	r.GET("/", controllers.Health)

	v1 := r.Group("/api/v1")
	{
		v1.POST("/register", controllers.Register)
		v1.POST("/login", controllers.Login)
		v1.GET("/me", middlewares.JwtAuth(), controllers.Me)
	}

	r.Run()
}
