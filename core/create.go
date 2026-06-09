package core

import (
	"fmt"
	"os"
)

func Create(name string) error {
	fmt.Println("Creating server...")
	err := os.Mkdir(name, 0755)
	if err != nil {
		return err
	}

	err = os.WriteFile(name+"/eula.txt", []byte("eula=true"), 0644)
	if err != nil {
		return err
	}

	return nil
}
