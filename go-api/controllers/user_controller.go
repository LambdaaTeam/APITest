package controllers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Me(c *gin.Context) {
	c.String(http.StatusOK, "Me")
}
