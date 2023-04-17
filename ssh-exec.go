package main

import (
	"fmt"
	"net"
	"sync"

	"golang.org/x/crypto/ssh"
)

func main() {
	hosts := []string{"10.10.0.1"}

	var wg sync.WaitGroup
	for _, host := range hosts {
		wg.Add(1)
		go func(host string) {
			defer wg.Done()
			if err := runCommand(host); err != nil {
				fmt.Printf("Error on host %s: %s\n", host, err)
			}
		}(host)
	}
	wg.Wait()
}

// run
func runCommand(host string) error {
	config := &ssh.ClientConfig{
		User: "name",
		Auth: []ssh.AuthMethod{
			ssh.Password("pass"),
		},
		HostKeyCallback: anyHostKey(),
	}

	client, err := ssh.Dial("tcp", host+":22", config)
	if err != nil {
		return err
	}
	defer client.Close()

	session, err := client.NewSession()
	if err != nil {
		return err
	}
	defer session.Close()

	out, err := session.CombinedOutput("ls")
	if err != nil {
		return err
	}
	fmt.Printf("Host: %s\nOutput:\n%s\n", host, out)
	return nil
}

// handle return key
func anyHostKey() ssh.HostKeyCallback {
	return func(hostname string, remote net.Addr, key ssh.PublicKey) error {
		return nil
	}
}
