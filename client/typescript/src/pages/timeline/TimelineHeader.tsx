import React from 'react';
import { Link } from "react-router-dom";
import styled from 'styled-components';

import { Button, Section } from '../../components';

const Container = styled(Section)`
  margin-bottom: 40px;
`;

const TimelineHeader: React.FC = () => {
  return <Container row>
      <Link to={'/post'}>
        <Button>New +</Button>
      </Link>
  </Container>
};

export default TimelineHeader;
