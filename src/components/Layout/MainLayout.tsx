import React, { useState, useEffect } from 'react';
import { Sidebar } from './Sidebar';
import { Topbar } from './Topbar';
import './MainLayout.css';

interface MainLayoutProps {
  children: React.ReactNode;
  onSearch?: (query: string) => void;
  onFilterChange?: (filter: 'all' | 'movie' | 'tv' | 'music') => void;
  onSectionChange?: (section: string) => void;
  currentFilter?: 'all' | 'movie' | 'tv' | 'music';
  currentSection?: string;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ 
  children, 
  onSearch,
  onFilterChange,
  onSectionChange,
  currentFilter = 'all',
  currentSection
}) => {
  // Map filter to section for initial state
  const filterToSection: Record<'all' | 'movie' | 'tv' | 'music', string> = {
    'all': 'home',
    'movie': 'movies',
    'tv': 'tv',
    'music': 'music',
  };
  
  const [activeSection, setActiveSection] = useState(
    currentSection || filterToSection[currentFilter]
  );

  // Sync active section when currentFilter changes from parent
  useEffect(() => {
    if (currentSection) {
      setActiveSection(currentSection);
    } else {
      setActiveSection(filterToSection[currentFilter]);
    }
  }, [currentFilter, currentSection]);

  const handleSectionChange = (section: string) => {
    setActiveSection(section);
    console.log('Section changed to:', section);
    
    // Notify parent about section change
    if (onSectionChange) {
      onSectionChange(section);
    }

    // Map sections to filters
    const filterMap: Record<string, 'all' | 'movie' | 'tv' | 'music'> = {
      'home': 'all',
      'movies': 'movie',
      'tv': 'tv',
      'music': 'music',
    };
    
    if (onFilterChange && filterMap[section]) {
      onFilterChange(filterMap[section]);
    }
  };

  const handleSearch = (query: string) => {
    console.log('Search query:', query);
    if (onSearch) {
      onSearch(query);
    }
  };

  const getSectionTitle = (section: string): string => {
    const titles: Record<string, string> = {
      home: 'Home',
      movies: 'Movies',
      tv: 'TV Shows',
      music: 'Music',
      playlists: 'Playlists',
      collections: 'Collections',
      recent: 'Recently Added',
      watching: 'Continue Watching',
      unwatched: 'Unwatched',
    };
    return titles[section] || 'CineVault';
  };

  return (
    <div className="main-layout">
      <Sidebar 
        activeSection={activeSection} 
        onSectionChange={handleSectionChange}
      />
      
      <div className="main-layout__content">
        <Topbar 
          title={getSectionTitle(activeSection)}
          onSearch={handleSearch}
        />
        
        <main className="main-layout__main">
          {children}
        </main>
      </div>
    </div>
  );
};
