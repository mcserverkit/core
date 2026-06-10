# MCServerKit

A Go library for creating Minecraft servers. This library is used in MCServerKit's [CLI](https://github.com/mcserverkit/cli) and [GUI](https://github.com/mcserverkit/gui).

## Installing

```bash
go get mcserverkit.github.io
```

## Usage

```go
package main

import (
	"fmt"

	"mcserverkit.github.io"
)

func main() {
	err := mcserverkit.Install("1.21.1")
	if err != nil {
		fmt.Println("Error installing 1.21.1:", err)
		return
	}

	err = mcserverkit.Create("MyServer")
	if err != nil {
		fmt.Println("Error creating MyServer:", err)
		return
	}

	err = mcserverkit.Start("MyServer", "4G")
	if err != nil {
		fmt.Println("Error starting MyServer:", err)
		return
	}
}
```
