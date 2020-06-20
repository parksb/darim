import styled from "styled-components";

// Component borrowed from dmitrizzle
// https://gist.github.com/dmitrizzle/5883ca1f72f34398155eb2bc1d0ccfc6
const LoadingDots = styled.span`
  &::after {
    animation: ellipsis 1.25s infinite;
    content: "";
  }

  @keyframes ellipsis {
    0% {
      content: "";
    }
     25% {
      content: ".";
    }
    50% {
      content: "..";
    }
    75% {
      content: "...";
    }
  }
`;

export default LoadingDots;
