import React, { useEffect, useRef, useState } from 'react';
import './Dropdown.css';

export interface DropdownItem {
  id: string;
  label: string;
  icon?: React.ReactNode;
  disabled?: boolean;
  danger?: boolean;
  separator?: boolean;
}

interface DropdownProps {
  trigger: React.ReactNode;
  items: DropdownItem[];
  onSelect: (item: DropdownItem) => void;
  align?: 'left' | 'right';
  className?: string;
}

export const Dropdown: React.FC<DropdownProps> = ({
  trigger,
  items,
  onSelect,
  align = 'left',
  className = '',
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      document.addEventListener('keydown', handleEscape);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    };
  }, [isOpen]);

  const handleSelect = (item: DropdownItem) => {
    if (!item.disabled && !item.separator) {
      onSelect(item);
      setIsOpen(false);
    }
  };

  return (
    <div className={`dropdown ${className}`} ref={dropdownRef}>
      <div
        className="dropdown__trigger"
        onClick={() => setIsOpen(!isOpen)}
      >
        {trigger}
      </div>

      {isOpen && (
        <div className={`dropdown__menu dropdown__menu--${align}`}>
          {items.map((item) =>
            item.separator ? (
              <div key={item.id} className="dropdown__separator" />
            ) : (
              <button
                key={item.id}
                className={`dropdown__item ${
                  item.disabled ? 'dropdown__item--disabled' : ''
                } ${item.danger ? 'dropdown__item--danger' : ''}`}
                onClick={() => handleSelect(item)}
                disabled={item.disabled}
              >
                {item.icon && (
                  <span className="dropdown__item-icon">{item.icon}</span>
                )}
                <span className="dropdown__item-label">{item.label}</span>
              </button>
            )
          )}
        </div>
      )}
    </div>
  );
};
