package main

import (
	"fmt"

	_ "github.com/joho/godotenv/autoload"
)

func main() {
	c := loadConfig()
	fmt.Printf("%+v\n", c)
}
