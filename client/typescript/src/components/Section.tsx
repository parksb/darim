import React from 'react';
import styled from 'styled-components'

const Section = styled(({ row, top, bottom, ...other }) => <div {...other} />)`
  display: flex;
  flex-direction: ${props => props.row || 'column'};
  margin-top: ${props => props.top ? `${props.top}px` : 0};
  margin-bottom: ${props => props.bottom ? `${props.bottom}px` : 0};
`;

export default Section;
