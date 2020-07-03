import React, {useEffect, useState} from 'react';
import { BrowserRouter as Router, Switch, Redirect, Route } from 'react-router-dom';
import styled from 'styled-components';

import * as api from '../api/auth';
import { Header } from '../components';
import { Session } from '../models';
import { Timeline } from './timeline';
import { Login, Join, Token } from './auth';
import { Post } from './post';
import { Settings } from './settings';

const Container = styled.div`
  max-width: 800px;
  padding: 30px 0 30px 0;
  margin: auto;
  word-break: keep-all;
  font-family: sans-serif;
`;

const App: React.FC = () => {
  const [session, setSession] = useState<Session | null>(null);

  const load = async () => {
    const session = await api.fetchSession();
    setSession(session);
  };

  useEffect(() => {
    load();
  }, []);

  return (
    <Router>
      <Container>
        <Header session={session} />
        <Switch>
          <Route path='/join'>
            {session ? <Redirect to="/" /> : <Join />}
          </Route>
          <Route path='/token/:key'>
            {session ? <Redirect to="/" /> : <Token />}
          </Route>
          <Route path='/post/:id'>
            {session && <Post session={session} />}
          </Route>
          <Route path='/post'>
            {session && <Post session={session} />}
          </Route>
          <Route path='/settings'>
            {session ? <Settings sessionState={[session, setSession]} /> : <Redirect to='/' />}
          </Route>
          <Route path='/'>
            {session ? <Timeline session={session} /> : <Login session_state={[session, setSession]} />}
          </Route>
          <Redirect to='/' />
        </Switch>
      </Container>
    </Router>
  );
};

export default App;
