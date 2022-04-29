import React, { useEffect, useState } from 'react';
import styled from 'styled-components';

import Storage from '../../utils/storage';
import { getI18n } from '../../utils/i18n';
import { localStoragePrivateKey } from '../../constants';
import * as authApi from '../../api/auth';
import * as userApi from '../../api/user';
import { Button, Section, TextField } from '../../components';
import { getSaveStatusText, SaveStatus } from '../../utils/status';
import { Session, ActiveUserSession } from '../../models';

interface Props {
  userId: string;
  userEmail: string;
  session: Session;
}

const SectionTitle = styled.h2`
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 15px;
`;

const StrongText = styled.strong`
  font-weight: 700;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const NonBorderFullWidthTextField = styled(FullWidthTextField)`
  border-top: 0;
`;

const NonBorderButton = styled(Button)`
  border-top: 0;
`;

const SaveStatusText = styled.span`
  margin-top: 5px;
  font-size: 12px;
  color: #c0c0c0;
`;

const RefreshLink = styled.span`
  color: #5f8fff;
  text-decoration: none;
  cursor: pointer;
  margin-left: 5px;

  &:hover {
    text-decoration: underline;
  }
`;

const SecuritySettings: React.FC<Props> = ({ userId, userEmail, session }) => {
  const [newSecretKey, setNewSecretKey] = useState('');
  const [newSecretKeySaveStatus, setNewSecretKeySaveStatus] = useState(SaveStatus.NONE);
  const [newSecretKeyPassword, setNewSecretKeyPassword] = useState('');

  const [newPassword, setNewPassword] = useState('');
  const [newPasswordSaveStatus, setNewPasswordSaveStatus] = useState(SaveStatus.NONE);
  const [newPasswordPassword, setNewPasswordPassword] = useState('');

  const [activeUserSessions, setActiveUserSessions] = useState<ActiveUserSession[]>([]);

  const i18n = getI18n({
    secretKey: {
      ko: '비밀키',
      en: 'Secret key',
    },
    newSecretKey: {
      ko: '새 비밀키',
      en: 'New secret key',
    },
    newPassword: {
      ko: '새 비밀번호',
      en: 'New password',
    },
    oldPassword: {
      ko: '기존 비밀번호',
      en: 'Old password',
    },
    refresh: {
      ko: '새로고침',
      en: 'Refresh',
    },
    activeUserSessions: {
      ko: '로그인된 세션',
      en: 'Logged-in Sessions',
    },
    lastAccessedAt: {
      ko: '마지막 접속',
      en: 'Last accessed on',
    },
    unknownDevice: {
      ko: '알 수 없는 장치',
      en: 'Unknown device',
    },
  });

  const load = async () => {
    const fetchedActiveUserSessions = await authApi.fetchActiveUserSessions(session.accessToken);
    setActiveUserSessions(fetchedActiveUserSessions);
  };

  useEffect(() => {
    load();
  }, []);

  const saveNewPrivateKey = async () => {
    setNewSecretKeySaveStatus(SaveStatus.ONGOING);
    const result = await authApi.login(userEmail, newSecretKeyPassword);

    setNewSecretKey('');
    setNewSecretKeyPassword('');

    if (result) {
      Storage.set(localStoragePrivateKey, newSecretKey);
      setNewSecretKeySaveStatus(SaveStatus.SUCCESS);
    } else {
      setNewSecretKeySaveStatus(SaveStatus.FAILURE);
    }
  };

  const saveNewPassword = async () => {
    setNewPasswordSaveStatus(SaveStatus.ONGOING);
    const result = await authApi.login(userEmail, newPasswordPassword);

    if (result) {
      const updateResult = await userApi.updateUser(userId, session.accessToken, newPassword);

      setNewPassword('');
      setNewPasswordPassword('');

      if (updateResult) {
        setNewPasswordSaveStatus(SaveStatus.SUCCESS);
      } else {
        setNewPasswordSaveStatus(SaveStatus.FAILURE);
      }
    } else {
      setNewPasswordSaveStatus(SaveStatus.FAILURE);
    }
  };

  return <Section>
    <Section bottom={30}>
      <SectionTitle>{i18n.text('secretKey')}</SectionTitle>
      <Section row>
        <FullWidthTextField
          type='password'
          placeholder={i18n.text('newSecretKey')}
          value={newSecretKey}
          onChange={({ target: { value } }) => setNewSecretKey(value)}
          autoComplete='new-password'
        />
      </Section>
      <Section row>
        <NonBorderFullWidthTextField
          type='password'
          placeholder={i18n.text('password')}
          value={newSecretKeyPassword}
          onChange={({ target: { value } }) => setNewSecretKeyPassword(value)}
          autoComplete='off'
        />
        <NonBorderButton onClick={saveNewPrivateKey}>{i18n.text('save')}</NonBorderButton>
      </Section>
      <SaveStatusText>
        {getSaveStatusText(newSecretKeySaveStatus)}
        {newSecretKeySaveStatus === SaveStatus.SUCCESS && <RefreshLink onClick={() => location.reload()}>Refresh</RefreshLink>}
      </SaveStatusText>
    </Section>
    <Section bottom={30}>
      <SectionTitle>{i18n.text('password')}</SectionTitle>
      <FullWidthTextField
        type='password'
        placeholder={i18n.text('newPassword')}
        value={newPassword}
        onChange={({ target: { value } }) => setNewPassword(value)}
        autoComplete='new-password'
      />
      <Section row>
        <NonBorderFullWidthTextField
          type='password'
          placeholder={i18n.text('oldPassword')}
          value={newPasswordPassword}
          onChange={({ target: { value } }) => setNewPasswordPassword(value)}
          autoComplete='off'
        />
        <NonBorderButton onClick={saveNewPassword}>{i18n.text('save')}</NonBorderButton>
      </Section>
      <SaveStatusText>{getSaveStatusText(newPasswordSaveStatus)}</SaveStatusText>
    </Section>
    <Section bottom={30}>
      <SectionTitle>{i18n.text('activeUserSessions')}</SectionTitle>
      {activeUserSessions.map((activeUserSession) => <Section bottom={10} key={activeUserSession.last_accessed_at}>
          <StrongText>{activeUserSession.user_agent ? activeUserSession.user_agent : i18n.text('unknownDevice')}</StrongText>
          <time>{`${i18n.text('lastAccessedAt')}: ${new Date(activeUserSession.last_accessed_at)}`}</time>
        </Section>)}
    </Section>
  </Section>;
};

export default SecuritySettings;
