/* @refresh reload */
import { render } from 'solid-js/web'
import { Router, Route } from '@solidjs/router';
import './index.css'
import Landing from './pages/Landing.tsx'
import NotFound from './pages/NotFound.tsx'
import Calendar from './pages/Calendar.tsx';

const root = document.getElementById('root')

render(
  () => (
    <Router>
      <Route path="/" component={Landing} />
      <Route path="/cal" component={Calendar} />
      <Route path="*404" component={NotFound} />
    </Router>
  ),
  root!
);
