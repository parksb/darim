import React, {useEffect, useState} from 'react';
import { BrowserRouter as Router, Switch, Redirect, Route } from 'react-router-dom';
import styled from 'styled-components';

import * as api from '../api/auth';
import { Header, Container } from '../components';
import { Session } from '../models';
import { Timeline } from './timeline';
import { Login, Join, Token } from './auth';
import { Post } from './post';
import { Settings } from './settings';

const Wrapper = styled(Container)`
  padding: 30px 0 30px 0;
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
      <Wrapper fullWidth>
        <Container>
          <Header session={session} />
        </Container>
        <Switch>
          <Route path='/join'>
            <Container>
              {session ? <Redirect to="/" /> : <Join />}
            </Container>
          </Route>
          <Route path='/token/:key'>
            <Container>
              {session ? <Redirect to="/" /> : <Token />}
            </Container>
          </Route>
          <Route path='/post/:id'>
            <Container>
              {session && <Post session={session} />}
            </Container>
          </Route>
          <Route path='/post'>
            <Container>
              {session && <Post session={session} />}
            </Container>
          </Route>
          <Route path='/settings'>
            <Container>
              {session ? <Settings sessionState={[session, setSession]} /> : <Redirect to='/' />}
            </Container>
          </Route>
          <Route path='/'>
            {session ? <Timeline session={session} /> : <Login session_state={[session, setSession]} />}
          </Route>
          <Redirect to='/' />
        </Switch>
      </Wrapper>
    </Router>
  );
};

export default App;
