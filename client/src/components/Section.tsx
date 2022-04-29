import styled from 'styled-components';

export interface SectionProps {
  row?: boolean;
  fullHeight?: boolean;
  nowrap?: boolean;
  top?: number;
  bottom?: number;
}

export const Section = styled.div<SectionProps>`
  display: flex;
  flex: ${(props) => !props.row && props.fullHeight && 1};
  flex-wrap: ${(props) => (props.nowrap ? 'nowrap' : 'wrap')};
  flex-direction: ${(props) => (props.row ? 'row' : 'column')};
  max-width: 100%;
  margin-top: ${(props) => (props.top ? `${props.top}px` : 0)};
  margin-bottom: ${(props) => (props.bottom ? `${props.bottom}px` : 0)};
  line-height: 150%;
`;
