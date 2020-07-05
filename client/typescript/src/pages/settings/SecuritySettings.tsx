import React, { useState } from 'react';
import styled from "styled-components";

import * as api from '../../api/auth';
import { Button, Section, TextField } from '../../components';
import Secret from "../../utils/secret";
import I18n from "../../utils/i18n";

enum SaveStatus {
  NONE,
  FAILURE,
  SUCCESS,
  ONGOING,
}

const SectionTitle = styled.h2`
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 15px;
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

const SecuritySettings: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [newSecretKey, setNewSecretKey] = useState('');
  const [newSecretKeySaveStatus, setNewSecretKeySaveStatus] = useState(SaveStatus.NONE);

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
    save: {
      'ko-KR': '저장',
      'en-US': 'Save',
    },
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

  const applyNewPrivateKey = async () => {
    setNewSecretKeySaveStatus(SaveStatus.ONGOING);
    const result = await api.login(email, password);

    setEmail('');
    setPassword('');
    setNewSecretKey('');

    if (result) {
      Secret.setPrivateKeyToLocalStorage(newSecretKey);
      setNewSecretKeySaveStatus(SaveStatus.SUCCESS);
    } else {
      setNewSecretKeySaveStatus(SaveStatus.FAILURE);
    }
  };

  return <Section>
    <Section>
      <SectionTitle>{i18n.text('secretKey')}</SectionTitle>
      <Section row>
        <FullWidthTextField type='password' placeholder={i18n.text('newSecretKey')} value={newSecretKey} onChange={({ target: { value } }) => setNewSecretKey(value)} autoComplete='new-password' />
      </Section>
      <Section row>
        <NonBorderFullWidthTextField type='email' placeholder={i18n.text('email')} value={email} onChange={({ target: { value } }) => setEmail(value)} />
        <NonBorderFullWidthTextField type='password' placeholder={i18n.text('password')} value={password} onChange={({ target: { value } }) => setPassword(value)} autoComplete='off' />
        <NonBorderButton onClick={applyNewPrivateKey}>{i18n.text('save')}</NonBorderButton>
      </Section>
      <SaveStatusText>{getSaveStatusText(newSecretKeySaveStatus)}</SaveStatusText>
    </Section>
  </Section>;
};

export default SecuritySettings;
