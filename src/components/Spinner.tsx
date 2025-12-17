import React from 'react';
import './Spinner.css';

interface SpinnerProps {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  className?: string;
}

export const Spinner: React.FC<SpinnerProps> = ({
  size = 'md',
  className = '',
}) => {
  return (
    <div className={`spinner-wrapper ${className}`}>
      <div className={`spinner spinner--${size}`} />
    </div>
  );
};
