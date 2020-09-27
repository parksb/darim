import React from 'react';
import styled from 'styled-components';
import { Redirect, Switch, Route, Link, useRouteMatch } from 'react-router-dom';

import { getI18n } from '../../utils/i18n';
import * as api from '../../api/auth';
import { Tab, Section } from '../../components';
import { Session } from '../../models';
import ProfileSettings from './ProfileSettings';
import SecuritySettings from './SecuritySettings';
import { localStoragePrivateKey } from '../../constants';

interface Props {
  sessionState: [Session, React.Dispatch<React.SetStateAction<Session | null>>]
}

const UserAvatar = styled(({ src, ...other }) => <div {...other} />)`
  width: 130px;
  height: 130px;
  background-color: #c0c0c0;
  border-radius: 50%;
  background-image: url(${props => props.src || ''});
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
`;

const ProfileContainer = styled(Section)`
  align-items: center;
  text-align: center;
`;

const UserName = styled.h2`
  font-size: 24px;
  font-weight: 700;
`;

const UserEmail = styled.h3`
  margin-top: 5px;
  font-size: 16px;
  font-weight: 100;
  color: #a0a0a0;
`;

const SettingsTab = styled(Tab)`
  flex: 3;
`;

const SecuritySettingsTab = styled(SettingsTab)`
  border-left: 0;
`;

const SignOutTab = styled(SettingsTab)`
  border-left: 0;
`;

const ButtonLink = styled(Link)`
  display: contents;
`;

const Settings: React.FC<Props> = ({ sessionState }) => {
  const [session, setSession] = sessionState;
  const { path, url } = useRouteMatch();

  const i18n = getI18n({
    profileSettings: {
      ko: '프로필 설정',
      en: 'Profile settings',
    },
    securitySettings: {
      ko: '보안 설정',
      en: 'Security settings',
    },
    signOut: {
      ko: '로그아웃 ↗',
      en: 'Sign out ↗'
    }
  });

  const signOut = async () => {
    const result = await api.logout();
    if (result) {
      localStorage.removeItem(localStoragePrivateKey);
      setSession(null);
    }
  };

  return <Section>
    <ProfileContainer>
      <UserAvatar src={session.user_avatar_url} />
      <Section top={10}>
        <UserName>{session.user_name}</UserName>
        <UserEmail>{session.user_email}</UserEmail>
      </Section>
    </ProfileContainer>
    <Section top={30} row>
      <ButtonLink to={`${url}/profile`}>
        <SettingsTab>{i18n.text('profileSettings')}</SettingsTab>
      </ButtonLink>
      <ButtonLink to={`${url}/security`}>
        <SecuritySettingsTab>{i18n.text('securitySettings')}</SecuritySettingsTab>
      </ButtonLink>
      <SignOutTab onClick={() => signOut()}>{i18n.text('signOut')}</SignOutTab>
    </Section>
    <Section top={40}>
      <Switch>
        <Route path={`${path}/profile`}>
          <ProfileSettings userId={session.user_id || ''} setSession={setSession}/>
        </Route>
        <Route path={`${path}/security`}>
          <SecuritySettings userId={session.user_id || ''} userEmail={session.user_email || ''}  />
        </Route>
        <Redirect to={`${path}/profile`} />
      </Switch>
    </Section>
  </Section>;
};

export default Settings;
