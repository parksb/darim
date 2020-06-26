import React, { useState } from 'react';
import { Redirect, useParams } from "react-router-dom";
import styled from 'styled-components';

import * as api from '../../api/user';
import { Button, TextField, Section } from '../../components';

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const Token: React.FC = () => {
  const { key } = useParams();

  const [pin, setPin] = useState('');
  const [isVerified, setIsVerified] = useState(false);

  const verify = async () => {
    const result = await api.createUser(key, pin);
    if (result) {
      alert('Welcome :)\nPlease sign in to start writing!');
      setIsVerified(true);
    } else {
      alert('Failed to verify email');
    }
  };

  return <Container row>
    {isVerified && <Redirect to='/' />}
    <FullWidthTextField type='text' placeholder='Pin' value={pin} onChange={({ target: { value } }) => setPin(value)} />
    <Button onClick={verify}>Verify</Button>
  </Container>
};

export default Token;
