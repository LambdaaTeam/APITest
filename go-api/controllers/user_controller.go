package controllers

import (
	"go-api/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

func Me(c *gin.Context) {
	user := c.MustGet("user").(models.User)

	c.JSON(http.StatusOK, user.PublicUser())
}
