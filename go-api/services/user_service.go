package services

import (
	"context"
	"errors"
	"go-api/models"
	"go-api/pkg"

	"go.mongodb.org/mongo-driver/bson"
)

func Register(createUser models.CreateUser) (*models.PublicUser, error) {
	user := createUser.ToUser()
	_, err := pkg.GetCollection("users").InsertOne(context.TODO(), user)

	if err != nil {
		return nil, err
	}

	publicUser := user.PublicUser()

	return &publicUser, nil
}

func Login(loginUser models.LoginUser) (*models.PublicUser, error) {
	var user models.User
	err := pkg.GetCollection("users").FindOne(context.TODO(), bson.M{"email": loginUser.Email}).Decode(&user)

	if err != nil {
		return nil, err
	}

	if !pkg.IsPasswordValid(user.Password, loginUser.Password) {
		return nil, errors.New("invalid password")
	}

	publicUser := user.PublicUser()

	return &publicUser, nil
}
