import React from 'react';
import styled from 'styled-components';

import * as api from '../../api/auth';
import { Button, Section } from '../../components';
import { Session } from '../../models';

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

const Settings: React.FC<Props> = ({ sessionState }) => {
  const [session, setSession] = sessionState;

  const signOut = async () => {
    const result = await api.logout();
    if (result) {
      setSession(null);
    } else {
      alert('Failed to sign out');
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
      <SettingsButton>Profile settings</SettingsButton>
      <SettingsButton>Security settings</SettingsButton>
      <SettingsButton onClick={() => signOut()}>Sign out</SettingsButton>
    </SettingsButtonSection>
  </Section>;
};

export default Settings;
