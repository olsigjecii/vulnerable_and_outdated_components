# Vulnerable and Outdated Components in Rust 🦀

This guide demonstrates how to run, exploit, and fix a vulnerability related to outdated components using a sample Rust application.

## ⚙️ 1. Build and Run the Server

You'll need Rust and Cargo installed on your system.

**1. Clone the repository (or ensure the project files are in a local directory):**

**2. Build and run the server:**

```sh
cargo run
```

You should see the following output, and the server will be running at `http://127.0.0.1:8080`.

🚀 Server starting at [http://127.0.0.1:8080](http://127.0.0.1:8080)

---

### 💥 2. Demonstrate the Vulnerability

The application has a feature to fetch a resource from a user-provided URL. The backend includes a check to block requests to the AWS metadata service IP (`169.254.169.254`), a common target for Server-Side Request Forgery (SSRF) attacks.

Our vulnerable endpoint at `/vulnerable/fetch_resource` uses flawed parsing logic that can be tricked.

#### Step 1: Send a Normal, Safe Request

This will work as expected.

```sh
curl -X POST -H "Content-Type: application/json" \
-d '{"url": "[http://example.com/path](http://example.com/path)"}' http://127.0.0.1:8080/vulnerable/fetch_resource
```

**✔️ Expected Output:**

```
Resource from [http://example.com/path](http://example.com/path) would be fetched here. The vulnerable check was bypassed.
```

**Server Logs:**

```
Vulnerable check: Parsed host as 'example.com'
✅ SUCCESS: Check bypassed. The application would now fetch from: [http://example.com/path](http://example.com/path)
```

#### Step 2: Try to Access the Blocked IP Directly

The vulnerable check is smart enough to block a direct attempt.

```sh
curl -X POST
-H "Content-Type: application/json" \
-d '{"url": "http://169.254.169.254/latest/meta-data/"}' \
http://127.0.0.1:8080/vulnerable/fetch_resource
```

**❌ Expected Output:**

```
Access to 169.254.169.254 is forbidden.
```

**Server Logs:**

```
Vulnerable check: Parsed host as '169.254.169.254'
❌ BLOCKED: Access to AWS metadata IP is forbidden.
```

#### Step 3: Exploit the Flawed Parser

Now, we'll craft a malicious URL using the `username@host` syntax. The vulnerable parser incorrectly identifies the `username` part as the host, bypassing the filter.

```sh
curl -X POST -H "Content-Type: application/json" \
-d '{"url": "http://safelooking.com@169.254.169.254/latest/meta-data/"}' \
http://127.0.0.1:8080/vulnerable/fetch_resource
```

**✔️ Expected Output (Exploit Successful):**

```bash
Resource from http://safelooking.com@169.254.169.254/latest/meta-data/ would be fetched here. The vulnerable check was bypassed.
```

**Server Logs:**

```
Vulnerable check: Parsed host as 'safelooking.com'
✅ SUCCESS: Check bypassed. The application would now fetch from: http://safelooking.com@169.254.169.254/latest/meta-data/
```

As you can see, our check was successfully bypassed\! The application would now make a request to the forbidden IP, potentially leaking sensitive cloud credentials.

---

### 🛡️ 3. Demonstrate the Mitigation

The fix involves replacing the flawed, manual parsing logic with a robust, standard library. Our secure endpoint at `/secure/fetch_resource` uses the official `url` crate for parsing.

#### Step 1: Send the Malicious URL to the Secure Endpoint

Let's try the same exploit payload on the `/secure/fetch_resource` endpoint.

```sh
curl -X POST -H "Content-Type: application/json" \
-d '{"url": "http://safelooking.com@169.254.169.254/latest/meta-data/"}' \
http://127.0.0.1:8080/secure/fetch_resource
```

**❌ Expected Output (Exploit Blocked):**

```
Access to 169.254.169.254 is forbidden.
```

**Server Logs:**

```
Secure check: Parsed host as '169.254.169.254'
❌ BLOCKED: Access to AWS metadata IP is correctly forbidden.
```

Success\! The `url` crate correctly identified `169.254.169.254` as the true host, and our security check successfully blocked the request.

---

### 🔑 4. Key Takeaways & Prevention

1.  **Dependency Awareness**: Always be aware of the dependencies (and transitive dependencies) in your project. An unmaintained library is a security risk.
2.  **Use Security Scanners**: For Rust projects, `cargo audit` is an essential tool for checking your dependencies against a database of known security vulnerabilities. It's the equivalent of `npm audit` or `snyk test`.
    ```sh
    cargo install cargo-audit
    cargo audit
    ```
3.  **Keep Dependencies Updated**: Regularly update your dependencies to receive security patches. Timely patching is a vital part of security posture.
4.  **Prefer Standard Libraries**: For critical operations like cryptography, authentication, and URL parsing, always prefer battle-tested, standard libraries over implementing your own logic.

<!-- end list -->

```

```
