# README


```r

library(magrittr)
library(ggplot2)

jsonlite::read_json("performance.json") -> dat


dat$results %>% purrr::map_df(function(x){
  problem = x$command %>% 
    stringr::str_extract("(?<=/)[:alnum:]+$")
  data.frame(
    command = problem,
    mean_time = x$mean,
    std_time = x$stddev
  )
}) %>% 
  ggplot(aes(x = command,
             y = mean_time*1000,
             ymax = mean_time*1000 - std_time*1000,
             ymin = mean_time*1000 + std_time*1000)) + 
  geom_point() + 
  geom_errorbar(width = 0.2) + 
  theme_classic() + 
  labs(y = "miliseconds", x = "Problem")


```
