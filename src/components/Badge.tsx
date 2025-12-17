import React from 'react';
import './Badge.css';

interface BadgeProps {
  children: React.ReactNode;
  variant?: 'default' | 'success' | 'error' | 'warning' | 'info';
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = 'default',
  size = 'md',
  className = '',
}) => {
  const classes = [
    'badge',
    `badge--${variant}`,
    `badge--${size}`,
    className,
  ].filter(Boolean).join(' ');

  return <span className={classes}>{children}</span>;
};
