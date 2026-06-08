package main

import (
	"fmt"
	"os"
	"os/exec"
)

func create(world string) {
	fmt.Println("Creating server...")
	config := readConfig()
	cmd := exec.Command("java", "-jar", config.Jar)
	err := cmd.Run()

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	os.Mkdir(world, 0755)
}
