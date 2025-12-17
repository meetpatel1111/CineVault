# CineVault UI Components Documentation

## Design System

### Color Palette (Dark Theme)
- **Background**: `#0f0f0f` (primary), `#1a1a1a` (secondary), `#252525` (tertiary)
- **Text**: `#f6f6f6` (primary), `#b3b3b3` (secondary), `#808080` (tertiary)
- **Accent**: `#e50914` (Netflix-inspired red)
- **Status Colors**: Success (#46d369), Warning (#ffa500), Error (#f44336), Info (#2196f3)

### Typography
- **Font Family**: System fonts (-apple-system, Segoe UI, Roboto, etc.)
- **Sizes**: xs (12px), sm (14px), base (16px), lg (18px), xl (20px), 2xl (24px), 3xl (30px), 4xl (36px)
- **Weights**: normal (400), medium (500), semibold (600), bold (700)

### Spacing Scale
- **Space Units**: 1 (4px), 2 (8px), 3 (12px), 4 (16px), 5 (20px), 6 (24px), 8 (32px), 10 (40px), 12 (48px), 16 (64px)

### Border Radius
- **Sizes**: sm (4px), md (8px), lg (12px), xl (16px), full (9999px)

---

## Core Components

### Button
**Location**: `src/components/Button.tsx`

Versatile button component with multiple variants and states.

**Props**:
```typescript
interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  icon?: React.ReactNode;
  iconPosition?: 'left' | 'right';
  fullWidth?: boolean;
  loading?: boolean;
  // + all standard button HTML attributes
}
```

**Variants**:
- `primary`: Red accent background (main actions)
- `secondary`: Gray background with border (secondary actions)
- `ghost`: Transparent background (tertiary actions)
- `danger`: Red error background (destructive actions)

**Usage**:
```tsx
<Button onClick={handleClick}>Click Me</Button>
<Button variant="secondary" size="lg" icon={<Icon />}>
  With Icon
</Button>
<Button loading fullWidth>Loading...</Button>
```

---

### Input
**Location**: `src/components/Input.tsx`

Text input component with label, icons, and validation states.

**Props**:
```typescript
interface InputProps {
  label?: string;
  error?: string;
  helperText?: string;
  icon?: React.ReactNode;
  iconPosition?: 'left' | 'right';
  fullWidth?: boolean;
  // + all standard input HTML attributes
}
```

**Features**:
- Optional label
- Left/right icons
- Error state with message
- Helper text
- Full width option
- Focus states with accent color

**Usage**:
```tsx
<Input 
  label="Search"
  placeholder="Type to search..."
  icon={<SearchIcon />}
  iconPosition="left"
/>

<Input 
  label="Email"
  type="email"
  error="Invalid email address"
/>
```

---

### MediaCard
**Location**: `src/components/MediaCard.tsx`

Card component for displaying media items (movies, TV shows, music).

**Props**:
```typescript
interface MediaCardProps {
  title: string;
  year?: number;
  duration?: number; // in minutes
  posterUrl?: string;
  type: 'movie' | 'tv' | 'music';
  watched?: boolean;
  progress?: number; // 0-100
  rating?: number; // 0-10
  onClick?: () => void;
}
```

**Features**:
- Poster image with placeholder fallback
- Hover overlay with play button
- Watch status badge (✓)
- Progress bar for partially watched content
- Rating display with star icon
- Type-specific placeholder icons
- Responsive poster aspect ratio (2:3)
- Smooth animations and transitions

**Usage**:
```tsx
<MediaCard
  title="Inception"
  year={2010}
  duration={148}
  posterUrl="/posters/inception.jpg"
  type="movie"
  progress={65}
  rating={8.8}
  onClick={() => playMovie('inception')}
/>
```

---

### MediaGrid
**Location**: `src/components/MediaGrid.tsx`

Responsive grid layout for displaying multiple media cards.

**Props**:
```typescript
interface MediaGridProps {
  items: MediaItem[];
  onItemClick?: (item: MediaItem) => void;
  emptyMessage?: string;
}
```

**Features**:
- Responsive grid (auto-fill with minmax)
- Empty state with custom message
- Breakpoint-aware column counts:
  - Desktop (1400px+): 180px min width
  - Tablet (768-1400px): 140-160px
  - Mobile (480-768px): 120px
  - Small Mobile (<480px): 2 columns

**Usage**:
```tsx
<MediaGrid
  items={mediaItems}
  onItemClick={(item) => console.log(item)}
  emptyMessage="No media found. Add some files!"
/>
```

---

## Layout Components

### MainLayout
**Location**: `src/components/Layout/MainLayout.tsx`

Main application layout with sidebar and topbar.

**Props**:
```typescript
interface MainLayoutProps {
  children: React.ReactNode;
}
```

**Structure**:
- Sidebar (240px fixed width)
- Content area with Topbar and scrollable main
- Section state management
- Search functionality

**Usage**:
```tsx
<MainLayout>
  <YourContent />
</MainLayout>
```

---

### Sidebar
**Location**: `src/components/Layout/Sidebar.tsx`

Navigation sidebar with library sections and quick access.

**Props**:
```typescript
interface SidebarProps {
  activeSection?: string;
  onSectionChange?: (section: string) => void;
}
```

**Sections**:
- **Library**: Home, Movies, TV Shows, Music, Playlists, Collections
- **Quick Access**: Recently Added, Continue Watching, Unwatched
- **Footer**: Settings

**Features**:
- Active state highlighting with accent border
- Emoji icons for visual appeal
- Hover states
- Scrollable navigation area

---

### Topbar
**Location**: `src/components/Layout/Topbar.tsx`

Top navigation bar with search and actions.

**Props**:
```typescript
interface TopbarProps {
  title?: string;
  onSearch?: (query: string) => void;
}
```

**Features**:
- Dynamic title display
- Centered search bar (max 600px width)
- Action buttons: Scan Library, View Options, Profile
- Responsive design

---

## Responsive Design

### Breakpoints
- **Desktop**: 1400px+
- **Laptop**: 1024px - 1400px
- **Tablet**: 768px - 1024px
- **Mobile**: 480px - 768px
- **Small Mobile**: < 480px

### Mobile Adaptations
- Reduced spacing and font sizes
- Adjusted grid columns
- Stacked layouts for complex components
- Touch-friendly button sizes

---

## Animations & Transitions

### Standard Timings
- **Fast**: 150ms (hover states)
- **Base**: 250ms (general transitions)
- **Slow**: 350ms (complex animations)

### Common Animations
- **Card Hover**: Transform translateY(-4px) + shadow
- **Button Hover**: Transform translateY(-1px)
- **Spinner**: Rotate 360° in 0.6s

---

## Accessibility

### Focus States
- 2px solid accent color outline
- 2px offset for buttons (4px for buttons)
- Visible on all interactive elements

### Keyboard Navigation
- Tab through all interactive elements
- Enter/Space to activate buttons
- Escape to close modals (future)

### Color Contrast
- Text colors meet WCAG AA standards
- Focus indicators are highly visible
- Error states use color + icon/text

---

## Usage Examples

### Basic Page Layout
```tsx
import { MainLayout, MediaGrid } from './components';

function MoviesPage() {
  return (
    <MainLayout>
      <div className="app-content">
        <section className="app-section">
          <h2 className="app-section-title">All Movies</h2>
          <MediaGrid items={movies} />
        </section>
      </div>
    </MainLayout>
  );
}
```

### Custom Section with Actions
```tsx
<section className="app-section">
  <div className="app-section-header">
    <div>
      <h2 className="app-section-title">My Playlists</h2>
      <p className="app-section-subtitle">
        {playlists.length} playlists
      </p>
    </div>
    <Button onClick={createPlaylist} icon={<PlusIcon />}>
      New Playlist
    </Button>
  </div>
  <MediaGrid items={playlists} />
</section>
```

---

## Custom Scrollbar

All scrollable areas have custom styled scrollbars:
- Width: 12px
- Track: `--color-bg-secondary`
- Thumb: `--color-bg-hover` (hover: `--color-bg-active`)
- Rounded corners

---

## Future Components (Planned)

### High Priority
- [ ] **Modal**: Dialog/overlay component
- [ ] **Dropdown**: Select and context menus
- [ ] **Tabs**: Tabbed content navigation
- [ ] **Badge**: Status and notification badges
- [ ] **Tooltip**: Hover information displays
- [ ] **Progress Bar**: Standalone progress indicator
- [ ] **Loading Skeleton**: Content placeholders

### Medium Priority
- [ ] **Video Player**: Full-featured media player
- [ ] **Audio Player**: Music playback interface
- [ ] **Settings Panel**: Application configuration
- [ ] **File Browser**: Directory selection
- [ ] **Toast/Notification**: Temporary messages
- [ ] **Breadcrumbs**: Navigation hierarchy

### Low Priority
- [ ] **Slider**: Range input component
- [ ] **Switch/Toggle**: Boolean input
- [ ] **Checkbox**: Multiple selection
- [ ] **Radio**: Single selection
- [ ] **Accordion**: Expandable sections
- [ ] **Carousel**: Image/content slider

---

## Design Guidelines

### Component Composition
- Keep components small and focused
- Use composition over complex props
- Export both component and its types

### Styling Strategy
- CSS Modules for component-specific styles
- CSS Variables for theming
- Utility classes for common patterns
- BEM-like naming for clarity

### Performance
- Lazy load images with `loading="lazy"`
- Use React.memo for expensive components
- Virtualize long lists (future)
- Debounce search inputs

### File Organization
```
src/
├── components/
│   ├── Button.tsx
│   ├── Button.css
│   ├── Layout/
│   │   ├── Sidebar.tsx
│   │   └── Sidebar.css
│   └── index.ts (exports)
├── styles/
│   └── variables.css
├── App.tsx
└── styles.css (global)
```

---

## Testing Checklist

When adding new components:
- [ ] Works with keyboard navigation
- [ ] Accessible focus states
- [ ] Responsive across breakpoints
- [ ] Dark theme compatible
- [ ] Props validated with TypeScript
- [ ] Loading/error states handled
- [ ] Animations smooth (60fps)
- [ ] Works with mock data
