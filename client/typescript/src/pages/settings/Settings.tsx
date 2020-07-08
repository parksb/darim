import React from 'react';
import styled from 'styled-components';
import { Switch, Route, Link, useRouteMatch } from "react-router-dom";

import * as api from '../../api/auth';
import { Button, Section } from '../../components';
import { Session } from '../../models';
import SecuritySettings from "./SecuritySettings";
import I18n from "../../utils/i18n";

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

const UserInfoSection = styled(Section)`
  margin-top: 10px;
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

const SettingsButtonSection = styled(Section)`
  margin-top: 30px;
`;

const SettingsButton = styled(Button)`
  flex: 3;

   &:nth-child(n+2) {
    border-left: 0;
  }
`;

const SecuritySettingsButton = styled(SettingsButton)`
  border-left: 0;
`;

const ButtonLink = styled(Link)`
  display: contents;
`;

const SettingsSection = styled(Section)`
  margin-top: 40px;
`;

const Settings: React.FC<Props> = ({ sessionState }) => {
  const [session, setSession] = sessionState;
  const { path, url } = useRouteMatch();

  const i18n = new I18n({
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
      setSession(null);
    }
  };

  return <Section>
    <ProfileContainer>
      <UserAvatar src={session.user_avatar_url} />
      <UserInfoSection>
        <UserName>{session.user_name}</UserName>
        <UserEmail>{session.user_email}</UserEmail>
      </UserInfoSection>
    </ProfileContainer>
    <SettingsButtonSection row>
      <SettingsButton>{i18n.text('profileSettings')}</SettingsButton>
      <ButtonLink to={`${url}/security`}>
        <SecuritySettingsButton>{i18n.text('securitySettings')}</SecuritySettingsButton>
      </ButtonLink>
      <SettingsButton onClick={() => signOut()}>{i18n.text('signOut')}</SettingsButton>
    </SettingsButtonSection>
    <SettingsSection>
      <Switch>
        <Route path={`${path}/security`}>
          <SecuritySettings userId={session.user_id || ''} userEmail={session.user_email || ''}  />
        </Route>
      </Switch>
    </SettingsSection>
  </Section>;
};

export default Settings;
