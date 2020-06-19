import React from 'react';
import styled from 'styled-components'

const Section = styled(({ row, ...other }) => <div {...other} />)`
  display: flex;
  flex-direction: ${props => props.row || 'column'};
`;

export default Section;
