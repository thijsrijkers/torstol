# Orca

**Orca** is an open source, self-hosted infrastructure orchestration engine written in Go. Define your infrastructure, databases, applications, DNS, load balancers, and more in a declarative `.orca` file, and let Orca handle the rest.

Think of it as CloudFormation, but one you own, run yourself, and extend however you want.

---

## How it works

Orca runs as a long lived HTTP server on your infrastructure. You bootstrap it once using the CLI with an initial environment file. After that, everything is driven by HTTP request an `.orca` file to the server, and Orca will reconcile your desired state with reality.

```
orca init --file environment.orca   # Bootstrap your environment
orca serve                          # Start the Orca HTTP server

curl -X POST http://localhost:7070/apply \
  -H "Content-Type: application/x-orca" \
  --data-binary @my-service.orca    # Deploy a new service
```

No proprietary dashboards. No cloud lock in. Just Go, HTTP, and a file.

---

## The Orca File

An `.orca` file is a declarative YAML based configuration that describes the desired state of a resource or environment. It follows a simple `kind` + `spec` structure:

```yaml
kind: Database
metadata:
  name: my_postgres
spec:
  engine: postgres
  version: "15"
  storage: 20Gi
  replicas: 1
```

```yaml
kind: App
metadata:
  name: my_api
spec:
  image: ghcr.io/myorg/api:latest
  port: 8080
  replicas: 2
  env:
    - name: DATABASE_URL
      value: postgres://...
```

Orca reads the `kind` field and routes it to the correct provisioner under the hood.

---

## Design Principles

- **Self hosted first** — Orca runs on your hardware, your cloud, your rules.
- **File driven** — everything is defined in an `.orca` file. No hidden state in a UI.
- **HTTP native** — the engine is an API. Automate it, wrap it, integrate it.
- **Pluggable** — providers are first-class. Orca doesn't dictate where you run.
- **Go all the way** — single binary, fast startup, easy to deploy Orca itself.

---

## Roadmap

### Phase 1 — Core Engine
- [ ] Define the `.orca` file specification (YAML-based, `kind` + `spec` pattern)
- [ ] Build the CLI (`orca init`, `orca serve`, `orca apply`, `orca status`, `orca destroy`)
- [ ] Bootstrap environment from an initial `.orca` file via `orca init`
- [ ] Start the Orca HTTP server via `orca serve`
- [ ] Accept `.orca` file payloads over HTTP `POST /apply`
- [ ] Parse, validate, and route resource kinds to their provisioners
- [ ] Basic state tracking, know what has been created, updated, or removed

---

### Phase 2 — Database Provisioning
- [ ] Provision PostgreSQL instances
- [ ] Provision MySQL instances
- [ ] Provision Redis instances
- [ ] Support configurable storage size, version, and replica count
- [ ] Lifecycle management, create, update, and destroy
- [ ] Connection string output exposed after provisioning

---

### Phase 3 — Application Hosting
- [ ] Deploy containerized applications from an image registry
- [ ] Configure environment variables and secrets
- [ ] Set replica count and resource limits (CPU/memory)
- [ ] Rolling deploy support, update without downtime
- [ ] Health check configuration (HTTP or TCP)
- [ ] Restart policies and crash recovery

---

### Phase 4 — DNS Management
- [ ] Register and manage DNS records (A, CNAME, TXT, MX)
- [ ] Attach a domain to a deployed application automatically
- [ ] Support multiple DNS providers (Cloudflare, Route53, etc.) via pluggable drivers
- [ ] TTL configuration per record
- [ ] Automatic DNS cleanup on resource destroy

---

### Phase 5 — Load Balancer
- [ ] Create and configure HTTP/HTTPS load balancers
- [ ] Route traffic to one or more application instances
- [ ] Support round-robin and least-connections strategies
- [ ] SSL/TLS termination with automatic certificate provisioning (via Let's Encrypt)
- [ ] Health-check-aware routing, remove unhealthy instances automatically

---

### Phase 6 — Secrets & Configuration
- [ ] Secrets store, store and inject secrets into apps at deploy time
- [ ] Support referencing secrets inside `.orca` files via `${{ secret.name }}` syntax
- [ ] Encryption at rest for stored secrets
- [ ] Secret rotation support

---

### Phase 7 — Networking & Firewalls
- [ ] Define virtual private networks (VPNs/VPCs) and subnets
- [ ] Configure firewall rules, allow/deny by port, protocol, and CIDR
- [ ] Private networking between services (service discovery by name)
- [ ] Public/private toggle per service

---

### Phase 8 — Object Storage
- [ ] Provision and manage object storage buckets
- [ ] Configure bucket policies (public read, private, etc.)
- [ ] Mount buckets as volumes in application containers
- [ ] S3 compatible API support

---

### Phase 9 — Observability
- [ ] Expose resource status via `GET /status/:name`
- [ ] Structured event log per resource (created, updated, failed, destroyed)
- [ ] Metrics endpoint (`/metrics`) compatible with Prometheus
- [ ] Deploy time dry run mode, validate a file without applying it (`POST /plan`)

---

### Phase 10 — Provider System
- [ ] Pluggable provider architecture, write your own provisioner in Go
- [ ] Official providers: Docker, Kubernetes, DigitalOcean, Hetzner, AWS
- [ ] Provider registry, install providers via `orca provider add <name>`
- [ ] Provider versioning and pinning inside `.orca` files

---


## Contributing

Orca is in early development. Contributions, ideas, and feedback are very welcome. Open an issue or a pull request.

---

## License

Apache 2.0
