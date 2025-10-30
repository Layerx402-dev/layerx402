# Layerx402 Test Suite

Comprehensive test suite for the Layerx402 x402 payment infrastructure.

## 📂 Test Structure

```
__tests__/
├── api/                    # API endpoint tests
│   └── payment-verification.test.ts
├── components/             # React component tests
│   ├── stats-section.test.tsx
│   └── code-showcase.test.tsx
├── integration/            # Integration tests
│   └── payment-flow.test.ts
├── lib/                    # Utility function tests
│   └── payment-utils.test.ts
└── README.md
```

## 🧪 Test Categories

### 1. API Tests (`__tests__/api/`)

Tests for REST API endpoints and payment verification logic.

**Coverage:**
- Payment verification endpoint
- Payment retrieval by ID
- Rate limiting
- Error handling
- Multi-network support

**Run:**
```bash
npm test __tests__/api
```

### 2. Component Tests (`__tests__/components/`)

Unit tests for React components using React Testing Library.

**Coverage:**
- StatsSection rendering and animations
- CodeShowcase tab switching and code display
- Copy-to-clipboard functionality
- Responsive layouts
- Canvas animations

**Run:**
```bash
npm test __tests__/components
```

### 3. Integration Tests (`__tests__/integration/`)

End-to-end tests for complete payment workflows.

**Coverage:**
- Complete payment lifecycle
  - Escrow creation
  - Funding
  - Payment verification
  - Escrow release
  - Settlement processing
- Error handling and edge cases
- WebSocket real-time updates
- Performance and load testing

**Run:**
```bash
npm test __tests__/integration
```

### 4. Utility Tests (`__tests__/lib/`)

Tests for utility functions and helpers.

**Coverage:**
- Payment proof validation
- Wallet address validation (Solana, Ethereum)
- Fee calculation
- Amount formatting and parsing
- Payment expiry checks
- Hash generation

**Run:**
```bash
npm test __tests__/lib
```

## 🚀 Running Tests

### Run All Tests
```bash
npm test
```

### Run Tests in Watch Mode
```bash
npm test -- --watch
```

### Run Tests with Coverage
```bash
npm test -- --coverage
```

### Run Specific Test File
```bash
npm test payment-verification.test
```

### Run Tests by Pattern
```bash
npm test -- --testNamePattern="payment"
```

## 📊 Coverage Goals

| Metric | Target | Current |
|--------|--------|---------|
| Statements | 70% | - |
| Branches | 70% | - |
| Functions | 70% | - |
| Lines | 70% | - |

## 🔧 Configuration

### Jest Configuration (`jest.config.js`)

- **Test Environment**: jsdom (for React components)
- **Module Aliases**: `@/` maps to root directory
- **Coverage Collection**: All source files except node_modules, .next, etc.
- **Setup File**: `jest.setup.js` for global mocks

### Test Setup (`jest.setup.js`)

Global mocks and setup:
- IntersectionObserver
- ResizeObserver
- matchMedia
- requestAnimationFrame
- Console error suppression

## 📝 Writing Tests

### API Test Example

```typescript
describe('API Endpoint', () => {
  it('should return successful response', async () => {
    const response = await fetch('/api/endpoint');
    const data = await response.json();

    expect(response.ok).toBe(true);
    expect(data.success).toBe(true);
  });
});
```

### Component Test Example

```typescript
describe('Component', () => {
  it('should render correctly', () => {
    render(<MyComponent />);

    expect(screen.getByText('Hello')).toBeInTheDocument();
  });

  it('should handle user interaction', () => {
    render(<MyComponent />);

    const button = screen.getByRole('button');
    fireEvent.click(button);

    expect(screen.getByText('Clicked')).toBeInTheDocument();
  });
});
```

### Integration Test Example

```typescript
describe('Payment Flow', () => {
  it('should complete full payment cycle', async () => {
    // Create escrow
    const escrow = await createEscrow();

    // Verify payment
    const payment = await verifyPayment(escrow.id);

    // Settle
    const settlement = await settlePayment(payment.id);

    expect(settlement.status).toBe('completed');
  });
});
```

### Utility Test Example

```typescript
describe('Utility Function', () => {
  it('should validate input correctly', () => {
    expect(validateInput('valid')).toBe(true);
    expect(validateInput('invalid')).toBe(false);
  });
});
```

## 🎯 Best Practices

### 1. Test Naming
- Use descriptive names: `should validate payment proof format`
- Follow pattern: `should [expected behavior] when [condition]`

### 2. Test Organization
- Group related tests with `describe` blocks
- Use `beforeEach` / `afterEach` for setup/teardown
- Keep tests independent and isolated

### 3. Assertions
- One logical assertion per test
- Use specific matchers (`toBe`, `toEqual`, `toContain`)
- Test both happy path and error cases

### 4. Mocking
- Mock external dependencies (fetch, WebSocket, etc.)
- Use `jest.fn()` for function mocks
- Clear mocks between tests with `jest.clearAllMocks()`

### 5. Async Tests
- Always `await` async operations
- Use `waitFor` for eventual assertions
- Set appropriate timeouts for long operations

## 🐛 Debugging Tests

### Run Single Test
```bash
npm test -- -t "should verify payment"
```

### Enable Verbose Output
```bash
npm test -- --verbose
```

### Debug in VS Code
Add to `.vscode/launch.json`:
```json
{
  "type": "node",
  "request": "launch",
  "name": "Jest Debug",
  "program": "${workspaceFolder}/node_modules/.bin/jest",
  "args": ["--runInBand", "--no-cache"],
  "console": "integratedTerminal"
}
```

## 📚 Resources

- [Jest Documentation](https://jestjs.io/)
- [React Testing Library](https://testing-library.com/react)
- [Testing Best Practices](https://kentcdodds.com/blog/common-mistakes-with-react-testing-library)

## 🤝 Contributing

When adding new features:
1. Write tests first (TDD)
2. Ensure all tests pass
3. Maintain coverage above 70%
4. Update test documentation

## 📄 License

Tests are part of the Layerx402 project - Proprietary
