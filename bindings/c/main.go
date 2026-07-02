package main

/*
#include <stdbool.h>
*/
import "C"

import (
	"mcserverkit.github.io"
)

//export Install
func Install(version *C.char) *C.char {
	err := mcserverkit.Install(C.GoString(version))
	if err != nil {
		return C.CString(err.Error())
	}
	return nil
}

//export Create
func Create(name *C.char, eula C.bool) *C.char {
	err := mcserverkit.Create(C.GoString(name), bool(eula))
	if err != nil {
		return C.CString(err.Error())
	}
	return nil
}

//export Start
func Start(name *C.char, memory *C.char) *C.char {
	err := mcserverkit.Start(C.GoString(name), C.GoString(memory))
	if err != nil {
		return C.CString(err.Error())
	}
	return nil
}

func main() {}
