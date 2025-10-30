import { describe, it, expect, beforeEach, jest } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { StatsSection } from '@/components/stats-section';

// Mock IntersectionObserver
class MockIntersectionObserver {
  observe = jest.fn();
  disconnect = jest.fn();
  unobserve = jest.fn();
}

global.IntersectionObserver = MockIntersectionObserver as any;

// Mock canvas for globe animation
HTMLCanvasElement.prototype.getContext = jest.fn(() => ({
  scale: jest.fn(),
  clearRect: jest.fn(),
  fillStyle: '',
  beginPath: jest.fn(),
  arc: jest.fn(),
  fill: jest.fn(),
  strokeStyle: '',
  lineWidth: 0,
  moveTo: jest.fn(),
  lineTo: jest.fn(),
  stroke: jest.fn(),
})) as any;

describe('StatsSection Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should render the stats section', () => {
    render(<StatsSection />);

    expect(screen.getByText(/The backbone for/i)).toBeInTheDocument();
    expect(screen.getByText(/global commerce/i)).toBeInTheDocument();
  });

  it('should display all four stats', () => {
    render(<StatsSection />);

    expect(screen.getByText('$22K+')).toBeInTheDocument();
    expect(screen.getByText('99.99%')).toBeInTheDocument();
    expect(screen.getByText('45+')).toBeInTheDocument();
    expect(screen.getByText('13K+')).toBeInTheDocument();
  });

  it('should display stat descriptions', () => {
    render(<StatsSection />);

    expect(screen.getByText(/x402 volume processed/i)).toBeInTheDocument();
    expect(screen.getByText(/historical uptime/i)).toBeInTheDocument();
    expect(screen.getByText(/builders and innovators/i)).toBeInTheDocument();
    expect(screen.getByText(/instant verifications/i)).toBeInTheDocument();
  });

  it('should have proper gradient background', () => {
    const { container } = render(<StatsSection />);
    const section = container.querySelector('section');

    expect(section).toHaveClass('bg-gradient-to-br');
  });

  it('should render canvas for globe visualization', () => {
    const { container } = render(<StatsSection />);
    const canvas = container.querySelector('canvas');

    expect(canvas).toBeInTheDocument();
  });

  it('should setup IntersectionObserver for animations', () => {
    render(<StatsSection />);

    expect(MockIntersectionObserver.prototype.observe).toHaveBeenCalled();
  });

  it('should animate stats on intersection', async () => {
    const { container } = render(<StatsSection />);

    // Simulate intersection
    const observerCallback = (MockIntersectionObserver.prototype as any).observe.mock.calls[0];

    await waitFor(() => {
      expect(container.querySelector('[style*="opacity"]')).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('should have responsive grid layout', () => {
    const { container } = render(<StatsSection />);
    const grid = container.querySelector('.grid');

    expect(grid).toHaveClass('lg:grid-cols-2');
  });

  it('should render with dark theme colors', () => {
    const { container } = render(<StatsSection />);
    const section = container.querySelector('section');

    expect(section?.className).toContain('from-[#0a1628]');
  });
});
