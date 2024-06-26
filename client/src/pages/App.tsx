import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Switch, Redirect, Route } from 'react-router-dom';
import styled from 'styled-components';
import { Storage } from 'snowball-js';

import { localStoragePrivateKey } from '../constants';
import * as api from '../api/auth';
import { Header, Footer, Container, SecretKeyWarningBar } from '../components';
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
  font-family: 'Spoqa Han Sans Neo', 'Noto Sans CJK KR', 'Noto Sans', sans-serif;
  max-width: 100%;
`;

const HeaderContainer = styled(Container)`
  padding: 30px 20px 0 20px;
`;

const PaddingContainer = styled(Container)`
  padding: 0 20px 0 20px;
`;

const ContainerWithFooter: React.FC = ({ children }) => {
  return <PaddingContainer>
    {children}
    <Footer />
  </PaddingContainer>;
};

const App: React.FC = () => {
  const [session, setSession] = useState<Session | null>(null);
  const [isFetchingSession, setIsFetchingSession] = useState<boolean>(false);

  const load = async () => {
    setIsFetchingSession(true);
    const session = await api.fetchSession();
    setIsFetchingSession(false);
    setSession(session);
  };

  useEffect(() => {
    load();
  }, []);

  return (
    <Router>
      <Wrapper fullWidth>
        {session && !Storage.get(localStoragePrivateKey) && <SecretKeyWarningBar />}
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
          <Route path='/:viewMode/:year/:month'>
            {session ? <Timeline session={session} /> : <Redirect to='/' />}
          </Route>
          <Route path='/:viewMode'>
            {session ? <Timeline session={session} /> : <Redirect to='/' />}
          </Route>
          <Route path='/'>
            {session ? <Timeline session={session} /> : !isFetchingSession && (
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
