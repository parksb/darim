import React, {useEffect, useState} from 'react';
import { BrowserRouter as Router, Switch, Redirect, Route } from 'react-router-dom';
import styled from 'styled-components';

import * as api from './api';
import { Header } from '../components';
import { Session } from '../models';
import { Timeline } from './timeline';
import { Login, Join } from './auth';
import { Post } from './post';

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
            {!session ? <Join /> : <Redirect to="/" />}
          </Route>
          <Route path='/post/:id'>
            {session && <Post />}
          </Route>
          <Route path='/post'>
            {session && <Post />}
          </Route>
          <Route path='/'>
            {!session && <Login session_state={[session, setSession]} />}
            {session && <Timeline />}
          </Route>
          <Redirect to="/" />
        </Switch>
      </Container>
    </Router>
  );
};

export default App;
