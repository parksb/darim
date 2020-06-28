import React, { useState } from 'react';
import { Link, useParams } from "react-router-dom";
import styled from 'styled-components';
import * as CryptoJS from 'crypto-js';
import CopyToClipboard from 'react-copy-to-clipboard';

import * as api from '../../api/user';
import { Button, TextField, Section } from '../../components';

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const InfoSection = styled(Section)`
  line-height: 150%;
  margin-bottom: 20px;
`;

const KeySection = styled(Section)`
  padding: 15px;
  background-color: #f0f0f0;
  margin: 20px 0 20px;
`;

const CopyButton = styled(Button)`
  margin-left: 10px;
`;

const GoToSignInButton = styled(Button)`
  width: 100%;
`;

const Token: React.FC = () => {
  const { key } = useParams();

  const [pin, setPin] = useState('');
  const [secretKey, setSecretKey] = useState('');

  const verify = async () => {
    const publicKey = CryptoJS.lib.WordArray.random(512 / 8).toString();
    const privateKey = CryptoJS.lib.WordArray.random(512 / 8).toString();
    const encryptedPrivateKey = CryptoJS.AES.encrypt(privateKey, publicKey).toString();

    const result = await api.createUser(publicKey, key, pin);
    if (result) {
      localStorage.setItem(publicKey, encryptedPrivateKey);
      setSecretKey(encryptedPrivateKey);
    }
  };

  const getDownloadURLOfTextFile = (text: string) => {
    const blob = new Blob([text], { type: 'text/plain' });
    return URL.createObjectURL(blob);
  };

  return <Container>
    {!secretKey ? (
      <Section row>
        <FullWidthTextField type='text' placeholder='Pin' value={pin} onChange={({ target: { value } }) => setPin(value)} />
        <Button onClick={verify}>Verify</Button>
      </Section>
    ) : (
      <>
        <InfoSection>
          👋 Welcome to Darim! This is your secret key that will be used to encrypt your posts:
          <KeySection row>
            <a download='darim-secret-key.txt' href={getDownloadURLOfTextFile(secretKey)}>
              <Button>Download the key as file</Button>
            </a>
            <CopyToClipboard text={secretKey}>
              <CopyButton>Copy the key to clipboard</CopyButton>
            </CopyToClipboard>
          </KeySection>
          Don't lose your secret key. It is strongly recommended that you download the key file and store it in a secure place, or copy the key to somewhere else.
          Also, NEVER let anyone know your secret key.
        </InfoSection>
        <Link to='/'>
          <GoToSignInButton>
            Go to sign in ↗
          </GoToSignInButton>
        </Link>
      </>
    )}
  </Container>
};

export default Token;
