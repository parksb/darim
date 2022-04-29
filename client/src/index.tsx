import React from 'react';
/* eslint-disable import/no-unresolved */
import ReactDOM from 'react-dom/client';
import { Reset } from 'styled-reset';

import App from './pages/App';

const root = document.getElementById('root');
ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <Reset />
    <App />
  </React.StrictMode>,
);
