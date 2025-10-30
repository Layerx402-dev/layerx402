import { describe, it, expect, beforeEach, jest } from '@jest/globals';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { CodeShowcase } from '@/components/code-showcase';

// Mock clipboard API
Object.assign(navigator, {
  clipboard: {
    writeText: jest.fn(() => Promise.resolve()),
  },
});

describe('CodeShowcase Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should render the code showcase section', () => {
    render(<CodeShowcase />);

    expect(screen.getByText(/Your First x402 Payment Verification/i)).toBeInTheDocument();
    expect(screen.getByText(/Here's how to verify an x402 payment/i)).toBeInTheDocument();
  });

  it('should display all three language tabs', () => {
    render(<CodeShowcase />);

    expect(screen.getByText('cURL')).toBeInTheDocument();
    expect(screen.getByText('JavaScript')).toBeInTheDocument();
    expect(screen.getByText('Python')).toBeInTheDocument();
  });

  it('should default to JavaScript tab', () => {
    render(<CodeShowcase />);

    const jsTab = screen.getByText('JavaScript');
    expect(jsTab).toHaveClass('text-white');
    expect(jsTab).toHaveClass('border-primary');
  });

  it('should switch between language tabs', () => {
    render(<CodeShowcase />);

    const curlTab = screen.getByText('cURL');
    fireEvent.click(curlTab);

    expect(curlTab).toHaveClass('text-white');

    const pythonTab = screen.getByText('Python');
    fireEvent.click(pythonTab);

    expect(pythonTab).toHaveClass('text-white');
  });

  it('should display code content', () => {
    const { container } = render(<CodeShowcase />);

    const codeBlock = container.querySelector('code');
    expect(codeBlock).toBeInTheDocument();
    expect(codeBlock?.textContent).toContain('fetch');
    expect(codeBlock?.textContent).toContain('api.layerx402.dev');
  });

  it('should have copy button', () => {
    render(<CodeShowcase />);

    expect(screen.getByText('Copy')).toBeInTheDocument();
  });

  it('should copy code to clipboard', async () => {
    render(<CodeShowcase />);

    const copyButton = screen.getByText('Copy').closest('button');
    fireEvent.click(copyButton!);

    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalled();
      expect(screen.getByText('Copied!')).toBeInTheDocument();
    });
  });

  it('should reset copy state after timeout', async () => {
    jest.useFakeTimers();
    render(<CodeShowcase />);

    const copyButton = screen.getByText('Copy').closest('button');
    fireEvent.click(copyButton!);

    await waitFor(() => {
      expect(screen.getByText('Copied!')).toBeInTheDocument();
    });

    // Fast-forward time
    jest.advanceTimersByTime(2000);

    await waitFor(() => {
      expect(screen.getByText('Copy')).toBeInTheDocument();
    });

    jest.useRealTimers();
  });

  it('should display metrics cards', () => {
    render(<CodeShowcase />);

    expect(screen.getByText('1 API Call')).toBeInTheDocument();
    expect(screen.getByText('< 50ms')).toBeInTheDocument();
    expect(screen.getByText('99.99%')).toBeInTheDocument();
  });

  it('should have documentation link', () => {
    render(<CodeShowcase />);

    const link = screen.getByText(/Read the full documentation/i).closest('a');
    expect(link).toHaveAttribute('href', 'https://docs.layerx402.dev');
    expect(link).toHaveAttribute('target', '_blank');
  });

  it('should display different code for each language', () => {
    const { container, rerender } = render(<CodeShowcase />);

    // JavaScript
    let codeBlock = container.querySelector('code');
    const jsCode = codeBlock?.textContent;
    expect(jsCode).toContain('fetch');

    // Switch to cURL
    const curlTab = screen.getByText('cURL');
    fireEvent.click(curlTab);
    rerender(<CodeShowcase />);

    codeBlock = container.querySelector('code');
    const curlCode = codeBlock?.textContent;
    expect(curlCode).toContain('curl');

    // Switch to Python
    const pythonTab = screen.getByText('Python');
    fireEvent.click(pythonTab);
    rerender(<CodeShowcase />);

    codeBlock = container.querySelector('code');
    const pythonCode = codeBlock?.textContent;
    expect(pythonCode).toContain('requests');
  });

  it('should have dark code editor theme', () => {
    const { container } = render(<CodeShowcase />);

    const codeContainer = container.querySelector('.bg-\\[\\#1a1a1a\\]');
    expect(codeContainer).toBeInTheDocument();
  });

  it('should display syntax elements', () => {
    const { container } = render(<CodeShowcase />);

    const codeBlock = container.querySelector('code');
    expect(codeBlock?.textContent).toContain('payment_proof');
    expect(codeBlock?.textContent).toContain('BASE64_ENCODED_PAYMENT_PROOF');
    expect(codeBlock?.textContent).toContain('YOUR_WALLET_ADDRESS');
  });
});
