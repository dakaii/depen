<a name="readme-top"></a>
[![GitHub][github-shield]][github-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
[![Medium][medium-shield]][medium-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/dakaii/depen">
    <img src="https://sp-ao.shortpixel.ai/client/to_auto,q_glossy,ret_img,w_1500/https://boardroom.tv/wp-content/uploads/2023/02/PUMA-NFT-75-Anniversary.jpg" alt="Logo" width="600" height="338">
  </a>

  <h3 align="center">Depen</h3>

  <p align="center">
    An awesome solution for the dependency resolver assignment!
    <br />
    <a href="https://github.com/dakaii/depen/issues">Report Bug</a>
    ·
    <a href="https://github.com/dakaii/depen/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#additional-info">Additional Info</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- GETTING STARTED -->

## Getting Started

### Prerequisites

- Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/dakaii/depen.git
   ```
2. Run the tests
   ```sh
   cargo test
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Additional Info -->

## Additional Info

1. Is function fallible? If so, under what circumstances should the function fail?
Yes, the function will fail if there is a circular dependency, which can occur while traversing through transitive dependencies(i.e. dependencies of dependency). 

2. What are the pros and cons of using recursive code?
Recursion can, in some cases, make algorithms look simpler and enables programmers to create shorter and concise code; however, if not used properly, it can increase the memory usage significantly, and the function can raise a stack overflow error as a result. Recursive code is often harder to wrap your head around as well.


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Daiki Nakashita - [@LinkedIn](https://www.linkedin.com/in/daikinakashita/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[linkedin-shield]: https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white
[linkedin-url]: https://www.linkedin.com/in/daikinakashita/
[github-shield]: https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white
[github-url]: https://github.com/dakaii/
[medium-shield]: https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white
[medium-url]: https://dakaii.medium.com/
