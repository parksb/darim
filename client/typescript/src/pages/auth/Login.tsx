import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import * as api from '../../api/auth';
import { TextField, Button, Section, LoadingDots } from '../../components';
import { Session } from '../../models';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const SignUpButton = styled(Button)`
  border-left: 0;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const Login: React.FC<Props> = ({ session_state }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [session, setSession] = session_state;

  const [isSigning, setIsSigning] = useState(false);

  const login = async () => {
    setIsSigning(true);
    const result = await api.login(email, password);
    if (result) {
      setIsSigning(false);
      setSession(result);
    } else {
      setIsSigning(false);
    }
  };

  return <Container row>
    {!isSigning ? (
      <>
        <FullWidthTextField type='email' placeholder='Email' value={email} onChange={({ target: { value } }) => setEmail(value)} />
        <FullWidthTextField type='password' placeholder='Password' value={password} onChange={({ target: { value } }) => setPassword(value)}/>
        <Button onClick={login}>Sign in</Button>
        <Link to='/join'>
          <SignUpButton>Sign up ↗</SignUpButton>
        </Link>
      </>
    ) : (
      <>
        Signing with secure encryption algorithm
        <LoadingDots />
      </>
    )}
  </Container>
};

export default Login;
