# Static Assets for ALFA APY Documentation

This folder contains static assets for the ALFA APY documentation site that gets deployed to GitHub Pages.

## Files

- `index.html` - Main HTML template for the documentation site with cosmic DeFi theme

## Features

The HTML template includes:

- **Cosmic DeFi Theme**: Dark theme with purple (#a855f7) and green (#22c55e) accents
- **Responsive Design**: Mobile-first approach with adaptive sidebar
- **Markdown Rendering**: Client-side markdown to HTML conversion
- **Syntax Highlighting**: Code blocks with Prism.js
- **Cosmic Effects**: Particle animations and smooth transitions
- **Navigation**: Sidebar with all documentation sections

## Customization

To modify the documentation site:

1. Edit `static/index.html`
2. Update styles in the `<style>` section
3. Modify JavaScript functionality in the `<script>` section
4. Push changes to trigger automatic deployment

## Color Scheme

- **Primary Background**: #0f0f0f (Dark)
- **Secondary Background**: #1a1a1a (Dark Gray)
- **Accent Purple**: #a855f7 (Primary)
- **Accent Green**: #22c55e (Secondary)
- **Text Primary**: #ffffff (White)
- **Text Secondary**: #e5e7eb (Light Gray)

## Deployment

The workflow automatically:
1. Copies this HTML template to the build directory
2. Includes all documentation files
3. Deploys to GitHub Pages

No additional build steps required - just push changes to deploy!
