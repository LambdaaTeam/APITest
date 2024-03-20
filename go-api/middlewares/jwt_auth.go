package middlewares

import (
	"context"
	"go-api/models"
	"go-api/pkg"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson"
)

func JwtAuth() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization")

		if token == "" {
			c.JSON(401, gin.H{
				"error": "Unauthorized",
			})
			c.Abort()
			return
		}

		token = token[7:]
		userId, err := pkg.DecodeToken(token)

		if err != nil {
			c.JSON(401, gin.H{
				"error": "Unauthorized",
			})
			c.Abort()
			return
		}

		var user models.User
		err = pkg.GetCollection("users").FindOne(context.TODO(), bson.M{"_id": userId}).Decode(&user)

		if err != nil {
			c.JSON(404, gin.H{
				"error": "User not found",
			})
			c.Abort()
			return
		}

		c.Set("user", user)

		c.Next()
	}
}
