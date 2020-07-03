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
`;

const PublicKeySection = styled(KeySection)`
  margin-top: 20px;
`;

const PrivateKeySection = styled(KeySection)`
  margin: 10px 0 20px;
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
  const [privateKey, setPrivateKey] = useState('');
  const [publicKey, setPublicKey] = useState('');

  const verify = async () => {
    const generatedPublicKey = CryptoJS.lib.WordArray.random(512 / 8).toString();
    const generatedPrivateKey = CryptoJS.lib.WordArray.random(512 / 8).toString();
    const encryptedPrivateKey = CryptoJS.AES.encrypt(generatedPrivateKey, generatedPublicKey).toString();

    const result = await api.createUser(generatedPublicKey, key, pin);
    if (result) {
      localStorage.setItem('key', encryptedPrivateKey);
      setPrivateKey(encryptedPrivateKey);
      setPublicKey(generatedPublicKey);
    }
  };

  const getDownloadURLOfTextFile = (text: string) => {
    const blob = new Blob([text], { type: 'text/plain' });
    return URL.createObjectURL(blob);
  };

  return <Container>
    {!privateKey ? (
      <Section row>
        <FullWidthTextField type='text' placeholder='Pin' value={pin} onChange={({ target: { value } }) => setPin(value)} />
        <Button onClick={verify}>Verify</Button>
      </Section>
    ) : (
      <>
        <InfoSection>
          👋 Welcome to Darim! This is your public key and secret key that will be used to encrypt your posts:
          <PublicKeySection row>
            <a download='darim-public-key.txt' href={getDownloadURLOfTextFile(privateKey)}>
              <Button>Download the public key as file</Button>
            </a>
            <CopyToClipboard text={publicKey}>
              <CopyButton>Copy the public key to clipboard</CopyButton>
            </CopyToClipboard>
          </PublicKeySection>
          <PrivateKeySection row>
            <a download='darim-secret-key.txt' href={getDownloadURLOfTextFile(publicKey)}>
              <Button>Download the secret key as file</Button>
            </a>
            <CopyToClipboard text={privateKey}>
              <CopyButton>Copy the secret key to clipboard</CopyButton>
            </CopyToClipboard>
          </PrivateKeySection>
          Don't lose your public key and secret key. It is strongly recommended that you download the key files and store it in a secure place, or copy the keys to somewhere else.
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
