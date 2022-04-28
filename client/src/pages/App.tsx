import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Routes, Navigate, Route } from 'react-router-dom';
import styled from 'styled-components';

import Storage from '../utils/storage';
import { localStoragePrivateKey } from '../constants';
import * as userApi from '../api/user';
import * as authApi from '../api/auth';
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
  min-height: 100vh;
`;

const HeaderContainer = styled(Container)`
  padding: 30px 20px 0 20px;
`;

const PaddingContainer = styled(Container)`
  padding: 0 20px 0 20px;
`;

const ContainerWithFooter = ({ children }: { children: React.ReactNode }) => {
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
    const accessToken = await authApi.fetchAccessToken();
    if (accessToken) {
      const user = await userApi.fetchUser(accessToken);
      if (user) {
        const fetchedSession: Session = { user, accessToken };
        setIsFetchingSession(false);
        setSession(fetchedSession);
      } else {
        setIsFetchingSession(false);
      }
    } else {
      setIsFetchingSession(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  return (
    <Router>
      <Wrapper>
        {session && !Storage.get(localStoragePrivateKey) && <SecretKeyWarningBar />}
        <HeaderContainer>
          <Header session={session} />
        </HeaderContainer>
        <Routes>
          <Route
            path='/join/:key'
            element={<ContainerWithFooter>{session ? <Navigate to='/' /> : <Join />}</ContainerWithFooter>}
          />
          <Route
            path='/password_reset'
            element={<ContainerWithFooter>{session ? <Navigate to='/' /> : <PasswordReset />}</ContainerWithFooter>}
          />
          <Route
            path='/post/:id'
            element={<ContainerWithFooter>{session && <Post session={session} />}</ContainerWithFooter>}
          />
          <Route
            path='/post'
            element={<ContainerWithFooter>{session && <Post session={session} />}</ContainerWithFooter>}
          />
          <Route
            path='/settings/*'
            element={<ContainerWithFooter>{session ? <Settings sessionState={[session, setSession]} /> : <Navigate to='/' />}</ContainerWithFooter>}
          />
          <Route
            path='/static/*'
            element={<ContainerWithFooter><Static /></ContainerWithFooter>}
          />
          <Route
            path='/:viewMode/:year/:month'
            element={session ? <Timeline session={session} /> : <Navigate to='/' />}
          />
          <Route
            path='/:viewMode'
            element={session ? <Timeline session={session} /> : <Navigate to='/' />}
          />
          <Route
            path='/'
            element={session ? <Timeline session={session} /> : !isFetchingSession && (<ContainerWithFooter><Landing session_state={[session, setSession]} /></ContainerWithFooter>)}
          />
          <Route
            path='*'
            element={<Navigate to='/' />}
          />
        </Routes>
      </Wrapper>
    </Router>
  );
};

export default App;
