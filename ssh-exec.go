package main

import (
	"fmt"
	"net"
	"sync"

	"golang.org/x/crypto/ssh"
)

func main() {
	hosts := []string{"10.10.21.64"}

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

func runCommand(host string) error {
	config := &ssh.ClientConfig{
		User: "rui",
		Auth: []ssh.AuthMethod{
			ssh.Password("ksji@123"),
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

	outChan := make(chan string)
	errChan := make(chan error)

	go func() {
		out, err := session.CombinedOutput("ls")
		if err != nil {
			errChan <- err
			return
		}
		outChan <- string(out)
	}()

	select {
	case out := <-outChan:
		fmt.Printf("Host: %s\nOutput:\n%s\n", host, out)
		return nil
	case err := <-errChan:
		return err
	}
}

func anyHostKey() ssh.HostKeyCallback {
	return func(hostname string, remote net.Addr, key ssh.PublicKey) error {
		return nil
	}
}
