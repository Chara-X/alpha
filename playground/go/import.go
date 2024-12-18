package main

import (
	_ "github.com/agiledragon/gomonkey/v2"
	_ "golang.org/x/crypto/ssh"
	_ "k8s.io/api"
	_ "k8s.io/apimachinery"
	_ "k8s.io/client-go"
	_ "sigs.k8s.io/controller-runtime"
)
