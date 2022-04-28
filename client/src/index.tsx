import React from 'react';
import ReactDOM from 'react-dom/client';
import { Reset } from 'styled-reset'

import App from './pages/App';

const root = document.getElementById('root');
ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <Reset />
    <App />
  </React.StrictMode>,
);
