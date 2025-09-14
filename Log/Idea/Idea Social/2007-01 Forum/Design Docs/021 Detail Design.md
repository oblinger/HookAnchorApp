# Detail Design - Component Specifications

## Overview

This document provides detailed technical specifications for all components, libraries, and infrastructure required to build the political discussion synthesis forum based on NodeBB.

## System Components Architecture

```
Development Environment/
├── GitHub Repository
└── Local Dev Machine (macOS/Linux/Windows)

Containerization/
├── Docker Engine v24.0+
└── Docker Compose v2.20+

Application Stack/
├── Node.js v18.x LTS
├── NodeBB v3.5.x
└── Custom Plugin v1.0.0
    ├── Event Logger
    ├── State Manager
    └── Tribal System

Data Layer/
├── MongoDB v6.0 (Primary Database)
│   ├── NodeBB Collections
│   ├── Event Log Collection
│   └── State Snapshots Collection
└── Redis v7.2 (Cache & Sessions)
    ├── Session Storage
    ├── Real-time Data
    └── Cache Layer

Infrastructure/
├── Nginx v1.24 (Reverse Proxy)
│   ├── SSL Termination
│   ├── Load Balancing
│   └── Static Assets
└── AWS EC2 t3.small
    ├── 2 vCPUs
    ├── 2 GB RAM
    └── 20 GB SSD Storage
```

## Component Specifications

### Core Platform

#### NodeBB Forum Software
- **Version:** 3.5.x (latest stable)
- **Repository:** https://github.com/NodeBB/NodeBB
- **License:** GPL-3.0
- **Node.js Requirement:** 18.x or 20.x LTS
```json
{
  "nodebb": "^3.5.0",
  "node": ">=18.0.0 <21.0.0"
}
```

#### Node.js Runtime
- **Version:** 18.19.0 LTS (Hydrogen)
- **NPM Version:** 10.2.3
- **Purpose:** JavaScript runtime for NodeBB
- **Download:** https://nodejs.org/

### Database Components

#### MongoDB (Primary Database)
- **Version:** 6.0.12
- **Purpose:** Main data storage for NodeBB
- **Docker Image:** `mongo:6.0`
- **Configuration:**
```yaml
mongodb:
  image: mongo:6.0
  environment:
    MONGO_INITDB_ROOT_USERNAME: admin
    MONGO_INITDB_ROOT_PASSWORD: ${MONGO_PASSWORD}
    MONGO_INITDB_DATABASE: nodebb
  volumes:
    - mongo_data:/data/db
  ports:
    - "27017:27017"
```

#### Redis (Cache & Sessions)
- **Version:** 7.2-alpine
- **Purpose:** Session storage, real-time data, caching
- **Docker Image:** `redis:7.2-alpine`
- **Configuration:**
```yaml
redis:
  image: redis:7.2-alpine
  command: redis-server --appendonly yes
  volumes:
    - redis_data:/data
  ports:
    - "6379:6379"
```

### NodeBB Dependencies

#### Core NodeBB Modules
```json
{
  "dependencies": {
    "express": "^4.18.2",
    "socket.io": "^4.6.1",
    "passport": "^0.6.0",
    "bcryptjs": "^2.4.3",
    "mongodb": "^5.9.0",
    "redis": "^4.6.10",
    "nconf": "^0.12.0",
    "winston": "^3.11.0",
    "async": "^3.2.5",
    "validator": "^13.11.0",
    "jimp": "^0.22.10"
  }
}
```

#### Essential NodeBB Plugins
```json
{
  "nodebb-plugin-composer-default": "^10.2.0",
  "nodebb-theme-harmony": "^1.2.0",
  "nodebb-plugin-markdown": "^11.0.0",
  "nodebb-plugin-mentions": "^4.2.0",
  "nodebb-plugin-emoji": "^5.0.0",
  "nodebb-plugin-emoji-android": "^4.0.0",
  "nodebb-plugin-user-invitations": "^2.0.0",
  "nodebb-plugin-registration-notification": "^2.1.0"
}
```

### Custom Plugin Components

#### Political Forum Plugin
```json
{
  "name": "nodebb-plugin-political-forum",
  "version": "1.0.0",
  "description": "Political discussion synthesis features for NodeBB",
  "main": "library.js",
  "dependencies": {
    "lodash": "^4.17.21",
    "uuid": "^9.0.1",
    "diff-match-patch": "^1.0.5"
  },
  "nbbpm": {
    "compatibility": "^3.5.0"
  }
}
```

#### Plugin Structure
```
nodebb-plugin-political-forum/
├── package.json
├── plugin.json
├── library.js
├── lib/
│   ├── eventLogger.js
│   ├── stateManager.js
│   ├── customRoutes.js
│   ├── socketHandlers.js
│   └── tribalSystem.js
├── static/
│   ├── js/
│   │   ├── client.js
│   │   ├── treeView.js
│   │   └── noteEditor.js
│   ├── templates/
│   │   ├── nodeViewer.tpl
│   │   ├── feedView.tpl
│   │   └── partials/
│   └── scss/
│       └── political-forum.scss
└── tests/
    ├── eventLogger.test.js
    └── stateManager.test.js
```

### Infrastructure Components

#### Docker & Docker Compose
- **Docker Engine:** 24.0.7+
- **Docker Compose:** 2.20.0+
- **Purpose:** Container orchestration
- **Installation:**
```bash
# macOS (using Homebrew)
brew install docker docker-compose

# Linux
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
```

#### Nginx (Reverse Proxy)
- **Version:** 1.24.0
- **Purpose:** SSL termination, load balancing, static assets
- **Docker Image:** `nginx:1.24-alpine`
- **Configuration:**
```nginx
server {
    listen 80;
    server_name forum.example.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name forum.example.com;

    ssl_certificate /etc/nginx/ssl/cert.pem;
    ssl_certificate_key /etc/nginx/ssl/key.pem;

    location / {
        proxy_pass http://nodebb:4567;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    location /socket.io/ {
        proxy_pass http://nodebb:4567/socket.io/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### Development Tools

#### Version Control
- **Git:** 2.40+
- **GitHub Repository Structure:**
```
political-forum/
├── .github/
│   └── workflows/
│       └── ci.yml
├── docker/
│   ├── Dockerfile.nodebb
│   └── docker-compose.yml
├── plugin/
│   └── [plugin source code]
├── config/
│   ├── nodebb.config.json
│   └── nginx.conf
├── scripts/
│   ├── setup.sh
│   ├── backup.sh
│   └── restore.sh
├── docs/
│   └── [documentation files]
├── .env.example
├── .gitignore
└── README.md
```

#### Development Dependencies
```json
{
  "devDependencies": {
    "eslint": "^8.54.0",
    "eslint-config-nodebb": "^0.2.1",
    "mocha": "^10.2.0",
    "chai": "^4.3.10",
    "sinon": "^17.0.1",
    "nodemon": "^3.0.2",
    "prettier": "^3.1.0"
  }
}
```

### Environment Configuration

#### Environment Variables (.env)
```bash
# NodeBB Configuration
NODE_ENV=production
PORT=4567
URL=https://forum.example.com
SECRET=generate-random-secret-here

# Database Configuration
MONGO_HOST=mongodb
MONGO_PORT=27017
MONGO_USERNAME=nodebb
MONGO_PASSWORD=secure-password-here
MONGO_DATABASE=nodebb

# Redis Configuration
REDIS_HOST=redis
REDIS_PORT=6379
REDIS_PASSWORD=redis-password-here

# Plugin Configuration
TRIBAL_CATEGORIES=progressive,conservative,libertarian,socialist,centrist
EVENT_LOG_RETENTION_DAYS=90
SNAPSHOT_INTERVAL_HOURS=6

# Email Configuration (Optional)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=app-specific-password
```

### Cloud Infrastructure

#### AWS EC2 Instance
- **Instance Type:** t3.small
- **vCPUs:** 2
- **Memory:** 2 GB
- **Storage:** 20 GB EBS (gp3)
- **OS:** Ubuntu 22.04 LTS
- **Security Groups:**
  - HTTP (80) - Redirect to HTTPS
  - HTTPS (443) - Main access
  - SSH (22) - Admin access (IP restricted)

#### Backup Strategy
- **Database:** Daily MongoDB dumps to S3
- **Event Log:** Continuous replication
- **Uploads:** S3 sync for user uploads
- **Configuration:** Git repository

### Performance Specifications

#### System Requirements
- **Minimum RAM:** 2 GB
- **Recommended RAM:** 4 GB
- **CPU Cores:** 2 minimum
- **Disk Space:** 20 GB minimum
- **Network:** 100 Mbps minimum

#### Expected Performance
- **Concurrent Users:** 200-500
- **Page Load Time:** < 2 seconds
- **WebSocket Latency:** < 100ms
- **Event Processing:** 100 events/second
- **State Rebuild Time:** < 2 seconds for 10k events

## Installation Order

1. **Development Environment Setup**
   - Install Docker & Docker Compose
   - Clone repository from GitHub
   - Configure environment variables

2. **Database Setup**
   - Start MongoDB container
   - Start Redis container
   - Verify connections

3. **NodeBB Installation**
   - Pull NodeBB Docker image or clone repository
   - Run NodeBB setup
   - Configure admin account

4. **Plugin Development**
   - Create plugin structure
   - Link plugin to NodeBB
   - Start development server

5. **Production Deployment**
   - Set up AWS EC2 instance
   - Configure Nginx
   - Deploy with Docker Compose
   - Set up SSL certificates

## Next Steps

1. Set up local development environment
2. Install and configure NodeBB
3. Create basic plugin structure
4. Implement event logging system
5. Build derived state manager
6. Create custom UI components
7. Deploy to production