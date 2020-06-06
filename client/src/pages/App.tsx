import React, { useState } from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import styled from 'styled-components';

import { Header } from '../components';
import { Session } from '../models';

import { Timeline } from './timeline';
import { Login, Join } from './auth';

const Container = styled.div`
  max-width: 800px;
  padding: 30px 0 30px 0;
  margin: auto;
  word-break: keep-all;
  font-family: sans-serif;
`;

const App: React.FC = () => {
  const [session, setSession] = useState<Session | null>(null);

  return (
    <Router>
      <Container>
        <Header />
        <Switch>
          <Route path='/join'>
            {!session && <Join />}
          </Route>
          <Route path='/'>
            {!session && <Login session_state={[session, setSession]} />}
            {session && <Timeline />}
          </Route>
        </Switch>
      </Container>
    </Router>
  );
};

export default App;
