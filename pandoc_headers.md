---
title: AlGlobo
titlepage: true
author: |
  | Grupo D*
  | Federico del Mazo - 100029
  | Javier Di Santo - 101696
  | Camila Dvorkin - 101109
date: |
  | Facultad de Ingeniería, Universidad de Buenos Aires
  | [71.59] Técnicas de Programación Concurrente I
  |
  | 26 de octubre de 2021
colorlinks: true
linkcolor: purple
urlcolor: purple
papersize: a4
geometry: margin=2.5cm
header-includes: |
  \usepackage{fancyhdr}
  \pagestyle{fancy}
  \fancyhead[L]{Grupo D* \\ del Mazo - Di Santo - Dvorkin \\ 2021C2}
  \fancyhead[R]{[71.59] Técnicas de Programación Concurrente I \\ AlGlobo}

  \usepackage{listings}
  \lstset{
      breaklines=true,
      breakatwhitespace=true,
      basicstyle=\ttfamily\footnotesize,
      frame=l,
      framesep=12pt,
      xleftmargin=12pt,
  }
  \let\OldTexttt\texttt

include-before: \renewcommand{\texttt}[1]{\OldTexttt{\color{magenta}{#1}}}
---
