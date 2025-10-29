# Support

Get help with the Layerx402 API and connect with our team.

## Documentation

Before reaching out to support, check if your question is answered in our documentation:

- [Getting Started](../getting-started.md) - Quick start guide
- [API Reference](../api-reference/) - Complete API documentation
- [Error Handling](../error-handling.md) - Common errors and solutions
- [FAQ](faq.md) - Frequently asked questions
- [Examples](../examples/) - Code examples in multiple languages

## System Status

Check the current status of Layerx402 services:

**Status Page**: [layerx402.dev/status](https://layerx402.dev/status)

The status page shows:
- Current system status
- Scheduled maintenance
- Incident history
- Performance metrics

## Contact Support

### Email Support

**Email**: support@layerx402.dev

**Response Times**:
- Standard Plan: Within 24 hours
- Premium Plan: Within 12 hours
- Enterprise Plan: Within 4 hours (24/7 support)

### When Contacting Support

Please include:

1. **Your Plan Tier**: Standard, Premium, or Enterprise
2. **Issue Description**: Detailed description of the problem
3. **Steps to Reproduce**: What you did before the error occurred
4. **Error Messages**: Full error messages and codes
5. **Request Details**:
   - Request IDs (if available)
   - Transaction signatures
   - Timestamps
6. **Environment**: Programming language, library versions
7. **Expected vs Actual Behavior**: What you expected and what happened

### Example Support Email

```
Subject: SLIPPAGE_EXCEEDED Error on Buy Orders

Plan: Premium
Issue: Getting consistent SLIPPAGE_EXCEEDED errors

Description:
I'm attempting to buy tokens using the Lightning API but consistently
receiving SLIPPAGE_EXCEEDED errors even with 15% slippage.

Steps to reproduce:
1. POST to /api/trade/buy
2. Parameters: amount=0.01, slippage=15
3. Token: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v

Error response:
{
  "error": {
    "code": "SLIPPAGE_EXCEEDED",
    "requestedSlippage": 15,
    "actualSlippage": 18.2
  }
}

Timestamp: 2024-10-26 14:30:00 UTC
Request ID: req_abc123xyz

Expected: Trade to execute within 15% slippage
Actual: Consistent slippage exceeded errors

Environment: Node.js 18, using fetch API
```

## Community

### Twitter

Follow us for updates, tips, and announcements:

**Twitter**: [@layerx402](https://x.com/layerx402)

### GitHub

Report documentation issues:

**GitHub**: [github.com/layerx402/docs](https://github.com/layerx402/docs/issues)

## Enterprise Support

Enterprise plan customers receive:

- **24/7 Priority Support**: Round-the-clock assistance
- **Dedicated Account Manager**: Personal point of contact
- **Direct Slack/Discord Channel**: Real-time communication
- **Custom SLA**: Tailored service level agreements
- **Architecture Review**: Help designing your integration
- **Performance Optimization**: Guidance on best practices

**Contact**: enterprise@layerx402.dev

## Feature Requests

We love hearing your ideas! Submit feature requests via:

1. Email: support@layerx402.dev with subject "Feature Request"
2. Twitter: Tweet [@layerx402](https://x.com/layerx402)
3. GitHub: [github.com/layerx402/docs/issues](https://github.com/layerx402/docs/issues)

Include:
- Clear description of the feature
- Your use case
- Expected behavior
- Any relevant examples

## Bug Reports

Found a bug? Help us improve by reporting it:

**Email**: support@layerx402.dev (Subject: "Bug Report")

Include:
- Detailed description
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Request IDs or signatures
- Screenshots (if applicable)

## Feedback

We value your feedback! Let us know:

- What's working well
- What could be improved
- Documentation gaps
- API usability issues

**Email**: feedback@layerx402.dev

## Office Hours

Join our weekly office hours for live Q&A:

- **When**: Every Thursday at 2 PM PST
- **Where**: Announced on Twitter
- **Format**: Video call (link shared on Twitter)

## Resources

- **Website**: [layerx402.dev](https://layerx402.dev)
- **Status**: [layerx402.dev/status](https://layerx402.dev/status)
- **Documentation**: [docs.layerx402.dev](https://docs.layerx402.dev)
- **Twitter**: [@layerx402](https://x.com/layerx402)

## Security Issues

**DO NOT** report security vulnerabilities through public channels.

**Security Email**: security@layerx402.dev

We take security seriously:
- We'll acknowledge your report within 24 hours
- We'll keep you updated on progress
- We'll credit you (unless you prefer anonymity)
- We offer bug bounties for qualifying vulnerabilities

See our [Security Policy](https://layerx402.dev/security) for details.
