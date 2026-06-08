package main

import (
	"fmt"
	"os"
	"os/exec"
)

func create(world string) {
	fmt.Println("Creating server...")
	config := readConfig()

	err := os.WriteFile("eula.txt", []byte("eula=true"), 0644)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fmt.Println("create server.")

	cmd := exec.Command("java", "-jar", config.Jar)
	err = cmd.Run()

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	os.Mkdir(world, 0755)
}
