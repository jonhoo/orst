Implementation of various [sorting algorithms] in Rust from [this live
stream].

To benchmark and plot (you'll need [R] and [ggplot2]):

```console
$ cargo r --release > values.dat
$ R
t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# to plot # comparisons
ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + scale_y_log10()
# to plot runtime
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + scale_y_log10()
```

[sorting algorithms]: https://en.wikipedia.org/wiki/Sorting_algorithm
[this live stream]: https://www.youtube.com/watch?v=h4RkCyJyXmM
[R]: https://www.r-project.org/
[ggplot2]: https://ggplot2.tidyverse.org/
