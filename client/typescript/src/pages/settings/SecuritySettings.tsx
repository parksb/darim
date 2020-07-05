import React, { useState } from 'react';
import styled from "styled-components";

import * as authApi from '../../api/auth';
import * as userApi from '../../api/user';
import { Button, Section, TextField } from '../../components';
import Secret from "../../utils/secret";
import I18n from "../../utils/i18n";

enum SaveStatus {
  NONE,
  FAILURE,
  SUCCESS,
  ONGOING,
}

interface Props {
  userId: string;
  userEmail: string;
}

const SettingsSection = styled(Section)`
  margin-bottom: 30px;
`;

const SectionTitle = styled.h2`
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 15px;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
  height: 21px;
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

const SecuritySettings: React.FC<Props> = ({ userId, userEmail }) => {
  const [newSecretKey, setNewSecretKey] = useState('');
  const [newSecretKeySaveStatus, setNewSecretKeySaveStatus] = useState(SaveStatus.NONE);
  const [newSecretKeyPassword, setNewSecretKeyPassword] = useState('');

  const [newPassword, setNewPassword] = useState('');
  const [newPasswordSaveStatus, setNewPasswordSaveStatus] = useState(SaveStatus.NONE);
  const [newPasswordPassword, setNewPasswordPassword] = useState('');

  const i18n = new I18n({
    saveStatusOngoing: {
      'ko-KR': '저장 중...',
      'en-US': 'Saving...',
    },
    saveStatusSuccess: {
      'ko-KR': '✅ 저장되었습니다!',
      'en-US': '✅ Saved!',
    },
    saveStatusFailure: {
      'ko-KR': '❌ 저장에 실패했습니다',
      'en-US': '❌ Failed to save',
    },
    secretKey: {
      'ko-KR': '비밀키',
      'en-US': 'Secret key',
    },
    newSecretKey: {
      'ko-KR': '새 비밀키',
      'en-US': 'New secret key',
    },
    newPassword: {
      'ko-KR': '새 비밀번호',
      'en-US': 'New password',
    },
    oldPassword: {
      'ko-KR': '기존 비밀번호',
      'en-US': 'Old password',
    }
  });

  const getSaveStatusText = (status: SaveStatus) => {
    switch (status) {
      case SaveStatus.FAILURE:
        return i18n.text('saveStatusFailure');
      case SaveStatus.SUCCESS:
        return i18n.text('saveStatusSuccess');
      case SaveStatus.ONGOING:
        return i18n.text('saveStatusOngoing');
      default:
        return '';
    }
  };

  const saveNewPrivateKey = async () => {
    setNewSecretKeySaveStatus(SaveStatus.ONGOING);
    const result = await authApi.login(userEmail, newSecretKeyPassword);

    setNewSecretKey('');
    setNewSecretKeyPassword('');

    if (result) {
      Secret.setPrivateKeyToLocalStorage(newSecretKey);
      setNewSecretKeySaveStatus(SaveStatus.SUCCESS);
    } else {
      setNewSecretKeySaveStatus(SaveStatus.FAILURE);
    }
  };

  const saveNewPassword = async () => {
    setNewPasswordSaveStatus(SaveStatus.ONGOING);
    const result = await authApi.login(userEmail, newPasswordPassword);

    if (result) {
      const updateResult = await userApi.updateUser(userId, newPassword);

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
    <SettingsSection>
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
    </SettingsSection>
    <SettingsSection>
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
      <SaveStatusText>{getSaveStatusText(newSecretKeySaveStatus)}</SaveStatusText>
    </SettingsSection>
  </Section>;
};

export default SecuritySettings;
