import React, { useState } from 'react';
import styled from 'styled-components';

import * as api from '../../api/auth';
import {Button, TextField, Section, LoadingDots} from '../../components';

const Container = styled(Section)`
  margin-bottom: 30px;
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

const Join: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [avatarUrl, setAvatarUrl] = useState('');

  const [isSettingToken, setIsSettingToken] = useState(false);
  const [isSetToken, setIsSetToken] = useState(false);

  const setSignUpToken = async () => {
    setIsSettingToken(true);
    const result = await api.setSignUpToken(name, email, password, avatarUrl.trim());

    if (result) {
      setIsSettingToken(false);
      setIsSetToken(true);
    } else {
      setIsSettingToken(false);
    }
  };

  return <Container>
    {isSetToken ? (
      `Please check your email (${email}) to verify your email address and activate your account`
    ) : (
      !isSettingToken ? (
        <>
          <Section row>
            <FullWidthTextField type='email' placeholder='Email' value={email} onChange={({ target: { value } }) => setEmail(value)} />
            <FullWidthTextField type='password' placeholder='Password' value={password} onChange={({ target: { value } }) => setPassword(value)}/>
            <FullWidthTextField type='text' placeholder='Name' value={name} onChange={({ target: { value } }) => setName(value)} />
          </Section>
          <Section row>
            <NonBorderFullWidthTextField type='url' placeholder='Avatar URL' value={avatarUrl} onChange={({ target: { value } }) => setAvatarUrl(value)} />
            <NonBorderButton onClick={setSignUpToken}>Create account</NonBorderButton>
          </Section>
        </>
      ) : (
        <Section row>
          Signing up with secure encryption algorithm
          <LoadingDots />
        </Section>
      )
    )}
  </Container>
};

export default Join;
