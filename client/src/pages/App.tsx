import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Switch, Redirect, Route } from 'react-router-dom';
import styled from 'styled-components';

import * as api from '../api/auth';
import { Header, Footer, Container } from '../components';
import { Session } from '../models';
import { Timeline } from './timeline';
import { Join, PasswordReset } from './auth';
import { Landing } from './landing';
import { Post } from './post';
import { Settings } from './settings';
import { Static } from './static';

const Wrapper = styled(Container)`
  flex: 1;
  word-break: keep-all;
  font-family: sans-serif;
`;

const HeaderContainer = styled(Container)`
  padding-top: 30px;
`;

const ContainerWithFooter: React.FC = ({ children }) => {
  return <Container>
    {children}
    <Footer />
  </Container>;
};

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
        <HeaderContainer>
          <Header session={session} />
        </HeaderContainer>
        <Switch>
          <Route path='/join/:key'>
            <ContainerWithFooter>
              {session ? <Redirect to="/" /> : <Join />}
            </ContainerWithFooter>
          </Route>
          <Route path='/password_reset'>
            <ContainerWithFooter>
              {session ? <Redirect to="/" /> : <PasswordReset />}
            </ContainerWithFooter>
          </Route>
          <Route path='/post/:id'>
            <ContainerWithFooter>
              {session && <Post session={session} />}
            </ContainerWithFooter>
          </Route>
          <Route path='/post'>
            <ContainerWithFooter>
              {session && <Post session={session} />}
            </ContainerWithFooter>
          </Route>
          <Route path='/settings'>
            <ContainerWithFooter>
              {session ? <Settings sessionState={[session, setSession]} /> : <Redirect to='/' />}
            </ContainerWithFooter>
          </Route>
          <Route path='/static'>
            <ContainerWithFooter>
              <Static />
            </ContainerWithFooter>
          </Route>
          <Route path='/'>
            {session ? <Timeline session={session} /> : (
              <ContainerWithFooter>
                <Landing session_state={[session, setSession]} />
              </ContainerWithFooter>
            )}
          </Route>
          <Redirect to='/' />
        </Switch>
      </Wrapper>
    </Router>
  );
};

export default App;
