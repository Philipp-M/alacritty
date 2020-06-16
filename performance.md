## ligature branch

vtebench bytes: 30,000,000

 Performance counter stats for 'cat scrolling.vte' (5 runs):

       4317.121510      task-clock:u (msec)       #    0.993 CPUs utilized            ( +-  0.19% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                54      page-faults:u             #    0.012 K/sec                    ( +-  0.75% )
         1,697,438      cycles:u                  #    0.000 GHz                      ( +-  1.90% )
           266,640      instructions:u            #    0.16  insn per cycle           ( +-  0.00% )
            61,669      branches:u                #    0.014 M/sec                    ( +-  0.00% )
             4,687      branch-misses:u           #    7.60% of all branches          ( +-  1.30% )

            4.3466 +- 0.0105 seconds time elapsed  ( +-  0.24% )

vtebench bytes: 100,000,000

 Performance counter stats for 'cat alt-screen-random-write.vte' (5 runs):

        454.356267      task-clock:u (msec)       #    0.302 CPUs utilized            ( +-  0.06% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                54      page-faults:u             #    0.118 K/sec                    ( +-  0.91% )
         2,924,607      cycles:u                  #    0.006 GHz                      ( +-  1.45% )
           381,985      instructions:u            #    0.13  insn per cycle           ( +-  0.00% )
            91,574      branches:u                #    0.202 M/sec                    ( +-  0.00% )
             7,166      branch-misses:u           #    7.83% of all branches          ( +-  1.90% )

           1.50589 +- 0.00999 seconds time elapsed  ( +-  0.66% )

vtebench bytes: 30,000,000

 Performance counter stats for 'cat scrolling-in-region.vte' (5 runs):

       4334.702997      task-clock:u (msec)       #    0.997 CPUs utilized            ( +-  0.17% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                53      page-faults:u             #    0.012 K/sec                    ( +-  0.75% )
         1,678,038      cycles:u                  #    0.000 GHz                      ( +-  4.07% )
           266,656      instructions:u            #    0.16  insn per cycle           ( +-  0.00% )
            61,672      branches:u                #    0.014 M/sec                    ( +-  0.00% )
             4,690      branch-misses:u           #    7.60% of all branches          ( +-  1.10% )

            4.3494 +- 0.0115 seconds time elapsed  ( +-  0.26% )

## master branch

vtebench bytes: 30,000,000

 Performance counter stats for 'cat scrolling.vte' (5 runs):

       4510.300996      task-clock:u (msec)       #    0.990 CPUs utilized            ( +-  0.44% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                54      page-faults:u             #    0.012 K/sec                    ( +-  0.70% )
         1,912,596      cycles:u                  #    0.000 GHz                      ( +-  3.33% )
           266,726      instructions:u            #    0.14  insn per cycle           ( +-  0.00% )
            61,695      branches:u                #    0.014 M/sec                    ( +-  0.00% )
             4,728      branch-misses:u           #    7.66% of all branches          ( +-  2.18% )

            4.5571 +- 0.0312 seconds time elapsed  ( +-  0.69% )

vtebench bytes: 100,000,000

 Performance counter stats for 'cat alt-screen-random-write.vte' (5 runs):

        455.743254      task-clock:u (msec)       #    0.299 CPUs utilized            ( +-  0.24% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                54      page-faults:u             #    0.118 K/sec                    ( +-  0.95% )
         2,839,043      cycles:u                  #    0.006 GHz                      ( +-  1.64% )
           381,985      instructions:u            #    0.13  insn per cycle           ( +-  0.00% )
            91,574      branches:u                #    0.201 M/sec                    ( +-  0.00% )
             6,990      branch-misses:u           #    7.63% of all branches          ( +-  1.24% )

           1.52661 +- 0.00374 seconds time elapsed  ( +-  0.24% )

vtebench bytes: 30,000,000

 Performance counter stats for 'cat scrolling-in-region.vte' (5 runs):

       4247.071459      task-clock:u (msec)       #    0.997 CPUs utilized            ( +-  0.55% )
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
                53      page-faults:u             #    0.012 K/sec                    ( +-  0.93% )
         1,742,107      cycles:u                  #    0.000 GHz                      ( +-  2.26% )
           266,656      instructions:u            #    0.15  insn per cycle           ( +-  0.00% )
            61,672      branches:u                #    0.015 M/sec                    ( +-  0.00% )
             4,601      branch-misses:u           #    7.46% of all branches          ( +-  1.89% )

            4.2583 +- 0.0243 seconds time elapsed  ( +-  0.57% )

