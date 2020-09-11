#!/usr/bin/env Rscript

library(tidyverse)
path <- commandArgs(trailingOnly=TRUE)[1]
data <- read_csv(path)
ggplot(data) +
    geom_line(aes(x = logsize, y = time, color = fft)) +
    scale_y_continuous(trans = "log2") +
    scale_x_continuous(trans = "log2")

