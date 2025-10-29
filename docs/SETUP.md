# Documentation Setup

This guide explains how to publish the Layerx402 API documentation to GitBook.

## Documentation Structure

```
docs/
├── .gitbook.yaml              # GitBook configuration
├── README.md                  # Documentation homepage
├── SUMMARY.md                 # Table of contents
├── getting-started.md         # Quick start guide
├── authentication.md          # Authentication guide
├── rate-limits.md            # Rate limiting documentation
├── error-handling.md         # Error handling guide
├── api-reference/
│   ├── README.md             # API overview
│   ├── lightning-api.md      # Lightning API endpoints
│   ├── local-transaction-api.md  # Local transaction API
│   └── websocket-api.md      # WebSocket streaming API
├── examples/
│   ├── README.md             # Examples overview
│   ├── javascript.md         # JavaScript/TypeScript examples
│   ├── python.md             # Python examples
│   └── rust.md               # Rust examples
└── resources/
    ├── faq.md                # Frequently asked questions
    ├── changelog.md          # Version history
    └── support.md            # Support information
```

## Publishing to GitBook

### Step 1: Sign Up

1. Go to [GitBook.com](https://www.gitbook.com/)
2. Sign up or log in

### Step 2: Create a Space

1. Click "New Space"
2. Choose "Import from Git"
3. Select your Git provider (GitHub, GitLab, etc.)
4. Authorize GitBook to access your repository

### Step 3: Configure Integration

1. Select this repository
2. Set the root directory to `docs/`
3. GitBook will automatically detect `.gitbook.yaml`
4. Click "Import"

### Step 4: Publish

Your documentation will be live at `https://your-space.gitbook.io`

### Step 5: Set Custom Domain (Optional)

1. Go to your space settings
2. Navigate to "Custom Domain"
3. Add `docs.layerx402.dev`
4. Follow DNS configuration instructions

## DNS Configuration

Add a CNAME record:

```
Type: CNAME
Name: docs
Value: <your-gitbook-space>.gitbook.io
```

## Local Development

To preview the documentation locally:

```bash
# Install GitBook CLI
npm install -g gitbook-cli

# Navigate to docs directory
cd docs

# Install GitBook
gitbook install

# Serve locally (http://localhost:4000)
gitbook serve

# Build static site
gitbook build
```

## Auto-Sync

GitBook automatically syncs with your repository when you push changes. No manual updates needed!

## Updating Documentation

1. Edit markdown files in `docs/`
2. Commit and push changes
3. GitBook automatically syncs and publishes

## Features

- ✅ Full-text search
- ✅ Mobile-responsive
- ✅ Dark/light mode
- ✅ Syntax highlighting
- ✅ Version control
- ✅ Analytics
- ✅ Custom domain support

## Support

For GitBook-specific help:
- [GitBook Documentation](https://docs.gitbook.com/)
- [GitBook Support](https://www.gitbook.com/support)
