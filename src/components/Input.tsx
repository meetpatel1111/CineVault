import React from 'react';
import './Input.css';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helperText?: string;
  icon?: React.ReactNode;
  iconPosition?: 'left' | 'right';
  fullWidth?: boolean;
}

export const Input: React.FC<InputProps> = ({
  label,
  error,
  helperText,
  icon,
  iconPosition = 'left',
  fullWidth = false,
  className = '',
  id,
  ...props
}) => {
  const inputId = id || `input-${Math.random().toString(36).substr(2, 9)}`;
  
  const wrapperClasses = [
    'input-wrapper',
    fullWidth ? 'input-wrapper--full-width' : '',
  ].filter(Boolean).join(' ');
  
  const containerClasses = [
    'input-container',
    icon ? `input-container--icon-${iconPosition}` : '',
    error ? 'input-container--error' : '',
    props.disabled ? 'input-container--disabled' : '',
  ].filter(Boolean).join(' ');

  return (
    <div className={wrapperClasses}>
      {label && (
        <label htmlFor={inputId} className="input-label">
          {label}
        </label>
      )}
      
      <div className={containerClasses}>
        {icon && iconPosition === 'left' && (
          <span className="input-icon input-icon--left">{icon}</span>
        )}
        
        <input
          id={inputId}
          className={`input ${className}`}
          {...props}
        />
        
        {icon && iconPosition === 'right' && (
          <span className="input-icon input-icon--right">{icon}</span>
        )}
      </div>
      
      {(error || helperText) && (
        <p className={`input-text ${error ? 'input-text--error' : ''}`}>
          {error || helperText}
        </p>
      )}
    </div>
  );
};
