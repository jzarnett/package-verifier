# package-verifier

Very simple server that responds with known-correct md5 checksums for a given
package name and version number. It's not very smart or complicated, but it
is intended as the remote server for testing ECE 459 Assignment 1. 

The server listens on port 4590 with the endpoints listed below:

# Endpoints

The server has one endpoint:
`GET /rest/v1/checksums/{package_name}/{version}`

For clarity, it takes two path parameters: `package_name` and `version`.

The server will check its registry and if it finds a matching combination of name
and version, it returns HTTP 200 with the body being the hash for that package.
If the server cannot find a corresponding combination of name and version, it will
return a HTTP 404. Other invalid requests may result in HTTP 400 or other error codes.

The implementation has a 25 - 250 ms pseudorandom delay built into it to simulate sending data over the actual internet. 
It is likely that for the assignment all the servers and clients will be in the same network 
(or at least geographically nearby) and the delay makes the test scenario a bit more plausible.

Usage example:
```
curl -v {server}:4590/rest/v1/checksums/{package_name}/{version}
```

Just fill in the parameters as required.

# Deployment

This requires rust to be installed. Use `cargo build` to compile; `cargo run` to execute.
It is suggested to run this in a `screen` session so it remains resident and so the output doesn't spam your console.
It binds to 0.0.0.0 so all ipv4 interfaces on port 4590.


The server looks for a file called `packages.csv` that contains the package information formatted
in csv, of course, with each row being `name,version,hash`. **Note that the Rust CSV parser assumes there are headers.**
If you don't put them then the first row will be ignored and that's unlikely to be what
you actually wanted.
