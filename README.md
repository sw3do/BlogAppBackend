# 🚀 Blog App Backend

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-orange?style=for-the-badge)](https://github.com/tokio-rs/axum)
[![MySQL](https://img.shields.io/badge/mysql-%2300f.svg?style=for-the-badge&logo=mysql&logoColor=white)](https://www.mysql.com/)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

[![CI/CD](https://github.com/sw3do/BlogAppBackend/actions/workflows/ci.yml/badge.svg)](https://github.com/sw3do/BlogAppBackend/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/sw3do/BlogAppBackend?style=flat-square)](LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/sw3do/BlogAppBackend?style=flat-square)](https://github.com/sw3do/BlogAppBackend/issues)
[![GitHub stars](https://img.shields.io/github/stars/sw3do/BlogAppBackend?style=flat-square)](https://github.com/sw3do/BlogAppBackend/stargazers)

A high-performance, modern blog backend API built with **Rust**, **Axum**, and **SQLx**. Features automatic database migrations, comprehensive testing, and Docker containerization.

## ✨ Features

- 🚀 **High Performance** - Built with Rust and Axum for maximum speed
- 🗄️ **Database Migrations** - Automatic SQLx migrations with version control
- 🔒 **Type Safety** - Compile-time guarantees with Rust's type system
- 🐳 **Containerized** - Docker support for easy deployment
- 🧪 **Well Tested** - Comprehensive test suite with CI/CD
- 📊 **RESTful API** - Clean, documented REST endpoints
- 🏷️ **Tag System** - JSON-based tagging with search capabilities
- 📝 **Rich Content** - Support for markdown content and excerpts
- 🔍 **Search & Filter** - Full-text search and tag filtering
- ⚡ **CORS Enabled** - Ready for frontend integration

## 🛠️ Tech Stack

- **Language**: Rust 🦀
- **Web Framework**: Axum
- **Database**: MySQL 8.0
- **ORM**: SQLx with compile-time checked queries
- **Serialization**: Serde
- **Async Runtime**: Tokio
- **Containerization**: Docker & Docker Compose
- **CI/CD**: GitHub Actions

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://www.docker.com/get-started) & Docker Compose
- [MySQL 8.0](https://dev.mysql.com/downloads/) (if running locally)

### 🐳 Docker Setup (Recommended)

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sw3do/BlogAppBackend.git
   cd BlogAppBackend
   ```

2. **Create environment file:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start with Docker Compose:**
   ```bash
   docker-compose up -d
   ```

4. **Access the API:**
   - API: http://localhost:3000
   - Health Check: http://localhost:3000/health
   - phpMyAdmin: http://localhost:8080

### 🔧 Local Development Setup

1. **Install dependencies:**
   ```bash
   # Install SQLx CLI
   cargo install sqlx-cli --no-default-features --features native-tls,mysql
   ```

2. **Setup database:**
   ```bash
   # Create database
   sqlx database create
   
   # Run migrations
   ./migrate.sh run
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

## 📚 API Documentation

### Base URL
```
http://localhost:3000/api
```

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/posts` | Get published posts with pagination |
| `GET` | `/posts/:slug` | Get post by slug |
| `POST` | `/posts` | Create new post |
| `GET` | `/admin/posts` | Get all posts (admin) |
| `GET` | `/admin/posts/:id` | Get post by ID |
| `PUT` | `/admin/posts/:id` | Update post |
| `DELETE` | `/admin/posts/:id` | Delete post |
| `GET` | `/config` | Get site configuration |
| `GET` | `/health` | Health check |

### Query Parameters

**GET /posts**
- `page` - Page number (default: 1)
- `limit` - Posts per page (default: 10, max: 50)
- `tag` - Filter by tag
- `search` - Search in title and content

### Example Requests

**Get posts with pagination:**
```bash
curl "http://localhost:3000/api/posts?page=1&limit=5"
```

**Search posts:**
```bash
curl "http://localhost:3000/api/posts?search=rust&tag=programming"
```

**Create a post:**
```bash
curl -X POST "http://localhost:3000/api/posts" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "content": "This is the content...",
    "excerpt": "A brief excerpt",
    "slug": "my-first-post",
    "author": "sw3do",
    "published": true,
    "tags": ["rust", "programming"]
  }'
```

## 🗄️ Database Schema

See [migrations/README.md](migrations/README.md) for detailed database schema documentation.

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🚀 Deployment

### Docker Deployment

1. **Build production image:**
   ```bash
   docker build -t blog-backend .
   ```

2. **Run with environment variables:**
   ```bash
   docker run -d \
     --name blog-backend \
     -p 3000:3000 \
     -e DATABASE_URL="mysql://user:pass@host:3306/db" \
     blog-backend
   ```

### GitHub Container Registry

Images are automatically built and pushed to GitHub Container Registry:

```bash
docker pull ghcr.io/sw3do/blog-backend:latest
```

## 🔧 Configuration

Environment variables:

| Variable | Description | Default |
|----------|-------------|----------|
| `DATABASE_URL` | MySQL connection string | Required |
| `SERVER_HOST` | Server bind address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `3000` |
| `SITE_NAME` | Blog site name | `My Blog` |
| `SITE_DESCRIPTION` | Site description | `A beautiful blog` |
| `SITE_URL` | Site URL | `http://localhost:4321` |
| `SITE_AUTHOR` | Default author | `Blog Author` |
| `POSTS_PER_PAGE` | Default pagination | `10` |
| `RUST_LOG` | Log level | `info` |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation
- Ensure CI passes
- Use conventional commits

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**sw3do**
- GitHub: [@sw3do](https://github.com/sw3do)
- Website: [sw3do.is-a.dev](https://sw3do.is-a.dev)

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Amazing web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization framework

---

⭐ **Star this repository if you find it helpful!**