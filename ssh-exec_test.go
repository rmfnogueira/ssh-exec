package main

import "testing"

func TestSSH(t *testing.T) {
	// Test that the function returns an error if the SSH connection fails
	_, err := ssh("nonexistent-host", "username", "password", "ls")
	if err == nil {
		t.Error("Expected an error, but got nil")
	}

	// Test that the function returns the output of a successful SSH command
	output, err := ssh("localhost", "username", "password", "echo hello")
	if err != nil {
		t.Errorf("Expected no error, but got %v", err)
	}
	expectedOutput := "hello\n"
	if output != expectedOutput {
		t.Errorf("Expected output %q, but got %q", expectedOutput, output)
	}
}

func TestRun(t *testing.T) {
	// Test that the function returns an error if the command fails
	_, err := runCommand("nonexistent-command")
	if err == nil {
		t.Error("Expected an error, but got nil")
	}

	// Test that the function returns the output of a successful command
	output, err := runCommand("echo hello")
	if err != nil {
		t.Errorf("Expected no error, but got %v", err)
	}
	expectedOutput := "hello\n"
	if output != expectedOutput {
		t.Errorf("Expected output %q, but got %q", expectedOutput, output)
	}
}
