# Changelog

All notable changes to the Layerx402 API will be documented here.

## [Unreleased]

### Coming Soon
- GraphQL API support
- Batch transaction endpoints
- Advanced order types (limit orders, stop-loss)
- Mobile SDKs (iOS, Android)

## [1.0.0] - 2024-10-26

### Added
- Initial public release
- Lightning API for quick trading integration
- Local Transaction API for self-custody
- WebSocket API for real-time data streaming
- Comprehensive API documentation
- Support for buy/sell operations
- Token information endpoints
- Portfolio tracking endpoints
- Rate limiting by plan tier
- Authentication via API keys
- Error handling with detailed error codes
- Transaction simulation before execution
- Priority fee optimization
- Slippage protection

### API Endpoints
- `POST /api/trade/buy` - Execute buy orders
- `POST /api/trade/sell` - Execute sell orders
- `POST /api/transaction/buy` - Build unsigned buy transactions
- `POST /api/transaction/sell` - Build unsigned sell transactions
- `GET /api/token/info/{mint}` - Get token information
- `GET /api/portfolio/{publicKey}` - Get portfolio holdings
- `WS /api/data` - WebSocket streaming endpoint

### Features
- Sub-25ms average execution time
- 99.99% uptime SLA
- Multiple plan tiers (Standard, Premium, Enterprise)
- Global infrastructure deployment
- Real-time price updates
- New token notifications
- Trade event streaming

## Version History

### How to Read This Changelog

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security improvements

### Staying Updated

- Follow [@layerx402](https://x.com/layerx402) for announcements
- Check this changelog regularly
- Subscribe to our newsletter for major updates

### Versioning

We use [Semantic Versioning](https://semver.org/):
- **Major** version for incompatible API changes
- **Minor** version for new functionality in a backward-compatible manner
- **Patch** version for backward-compatible bug fixes

### Deprecation Policy

When we deprecate features:
1. We announce the deprecation at least 90 days in advance
2. The feature continues to work during the deprecation period
3. We provide migration guides for affected users
4. We offer support for migration to new features

### API Versioning

Currently, we maintain a single version of the API. When we introduce breaking changes, we'll implement versioned endpoints (e.g., `/v2/trade/buy`).
