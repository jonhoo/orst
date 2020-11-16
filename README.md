Implementation of various [sorting algorithms] in Rust from [this live
stream].

Modify `MAX_ELEMENT` and `MIN_ELEMENT` in bench function to control the no. of elements (`n`) and benchmark different scenarios.

To benchmark and plot (you'll need [R] and [ggplot2]):

```console
$ cargo r --release > values.dat
$ R
t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# to plot # comparisons
ggplot(t, aes(n, comparisons, colour = algorithm, linetype = algorithm)) + geom_point(size = 0.4, show.legend = FALSE) + geom_smooth(method = "gam") + scale_y_log10()
# to plot runtime
ggplot(t, aes(n, time, colour = algorithm, linetype = algorithm)) + geom_point(size = 0.4, show.legend = FALSE) + geom_smooth(method = "gam") + scale_y_log10()
```

In case of errors with `geom_smooth()`, remove `method = "gam"`. This occurs if there are very few data points.

[sorting algorithms]: https://en.wikipedia.org/wiki/Sorting_algorithm
[this live stream]: https://www.youtube.com/watch?v=h4RkCyJyXmM
[r]: https://www.r-project.org/
[ggplot2]: https://ggplot2.tidyverse.org/
