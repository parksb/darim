import React, { useState } from 'react';
import styled from "styled-components";

import * as api from '../../api/auth';
import { Button, Section, TextField } from '../../components';
import Secret from "../../utils/secret";

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

  const getSaveStatusText = (status: SaveStatus) => {
    switch (status) {
      case SaveStatus.NONE:
        return '';
      case SaveStatus.FAILURE:
        return '❌ Failed to save';
      case SaveStatus.SUCCESS:
        return '✅ Saved!';
      case SaveStatus.ONGOING:
        return 'Saving...';
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
      <SectionTitle>Secret key</SectionTitle>
      <Section row>
        <FullWidthTextField type='password' placeholder='New secret key' value={newSecretKey} onChange={({ target: { value } }) => setNewSecretKey(value)} autoComplete='new-password' />
      </Section>
      <Section row>
        <NonBorderFullWidthTextField type='email' placeholder='Email' value={email} onChange={({ target: { value } }) => setEmail(value)} />
        <NonBorderFullWidthTextField type='password' placeholder='Password' value={password} onChange={({ target: { value } }) => setPassword(value)} autoComplete='off' />
        <NonBorderButton onClick={applyNewPrivateKey}>Save</NonBorderButton>
      </Section>
      <SaveStatusText>{getSaveStatusText(newSecretKeySaveStatus)}</SaveStatusText>
    </Section>
  </Section>;
};

export default SecuritySettings;
