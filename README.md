# Rushell

## Overview

**Rushell** is a dynamic reverse shell written in Rust that extracts the target IP address and port number directly from the executable's filename. By naming the executable in the format `192_168_10_100_4000`, Rushell will automatically connect back to the specified IP address (`192.168.10.100`) and port (`4000`) upon execution.

## Features

- **Dynamic Extraction**: Automatically parses IP and port from the executable filename.
- **Cross-Platform Compatibility**: Supports both Windows and Unix-like operating systems.
- **Interactive Shell**: Establishes an interactive shell allowing remote command execution.
