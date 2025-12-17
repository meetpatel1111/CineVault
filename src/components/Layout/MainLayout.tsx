import React, { useState } from 'react';
import { Sidebar } from './Sidebar';
import { Topbar } from './Topbar';
import './MainLayout.css';

interface MainLayoutProps {
  children: React.ReactNode;
  onSearch?: (query: string) => void;
  onFilterChange?: (filter: 'all' | 'movie' | 'tv' | 'music') => void;
  currentFilter?: 'all' | 'movie' | 'tv' | 'music';
}

export const MainLayout: React.FC<MainLayoutProps> = ({ 
  children, 
  onSearch,
  onFilterChange,
  currentFilter = 'all'
}) => {
  const [activeSection, setActiveSection] = useState('home');

  const handleSectionChange = (section: string) => {
    setActiveSection(section);
    console.log('Section changed to:', section);
    
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
