package models

type Claims struct {
	Sub string `json:"sub"`
	Exp int64  `json:"exp"`
}

func NewClaims(sub string) *Claims {
	return &Claims{
		Sub: sub,
		Exp: 24 * 7,
	}
}
