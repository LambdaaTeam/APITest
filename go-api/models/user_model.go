package models

import (
	"go-api/pkg"
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type User struct {
	ID           primitive.ObjectID `json:"_id" bson:"_id"`
	Name         string             `json:"name" bson:"name"`
	Email        string             `json:"email" bson:"email"`
	Password     []byte             `json:"password" bson:"password"`
	SchemaVerion uint8              `json:"schema_version" bson:"schema_version"`
	CreateAt     time.Time          `json:"create_at" bson:"create_at"`
	UpdatedAt    time.Time          `json:"updated_at" bson:"updated_at"`
}

type PublicUser struct {
	ID        string `json:"id"`
	Name      string `json:"name"`
	Email     string `json:"email"`
	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

type PublicUserWithToken struct {
	ID        string `json:"id"`
	Name      string `json:"name"`
	Email     string `json:"email"`
	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
	Token     string `json:"token"`
}

type CreateUser struct {
	Name     string `json:"name" binding:"required"`
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required"`
}

type LoginUser struct {
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required"`
}

func (u *User) PublicUser() PublicUser {
	return PublicUser{
		ID:        u.ID.Hex(),
		Name:      u.Name,
		Email:     u.Email,
		CreatedAt: u.CreateAt.String(),
		UpdatedAt: u.UpdatedAt.String(),
	}
}

func (u *User) PublicUserWithToken(token string) PublicUserWithToken {
	return PublicUserWithToken{
		ID:        u.ID.Hex(),
		Name:      u.Name,
		Email:     u.Email,
		CreatedAt: u.CreateAt.String(),
		UpdatedAt: u.UpdatedAt.String(),
		Token:     token,
	}
}

func (u *CreateUser) ToUser() User {
	return User{
		ID:           primitive.NewObjectID(),
		Name:         u.Name,
		Email:        u.Email,
		Password:     pkg.HashPassword(u.Password),
		SchemaVerion: 1,
		CreateAt:     time.Now(),
		UpdatedAt:    time.Now(),
	}
}
