import React, { useState } from 'react';
import { useParams } from 'react-router-dom';

import { Container } from '../../components';
import Verification from './Verification';
import Token from './Token';

const Join: React.FC = () => {
  const { key } = useParams<{ key?: string }>();

  const [privateKey, setPrivateKey] = useState('');
  const [publicKey, setPublicKey] = useState('');

  return <Container>
    {privateKey && publicKey ? (
      <Token publicKey={publicKey} privateKey={privateKey} />
    ) : (
      <Verification
        tokenKey={key}
        privateKeyState={[privateKey, setPrivateKey]}
        publicKeyState={[publicKey, setPublicKey]}
      />
    )}
  </Container>;
};

export default Join;
