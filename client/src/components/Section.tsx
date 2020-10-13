import React from 'react';
import styled from 'styled-components'

const Section = styled(({ row, top, bottom, fullHeight, ...other }) => <div {...other} />)`
  display: flex;
  flex: ${props => !props.row && props.fullHeight && 1};
  flex-wrap: ${props => props.row && 'wrap'};
  max-width: 100%;
  flex-direction: ${props => props.row || 'column'};
  margin-top: ${props => props.top ? `${props.top}px` : 0};
  margin-bottom: ${props => props.bottom ? `${props.bottom}px` : 0};
  line-height: 150%;
`;

export default Section;
