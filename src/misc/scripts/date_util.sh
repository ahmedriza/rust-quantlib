#!/bin/bash

# take a file containing lines such as the following:
#
# { make_pair(Date(19, Mar, 2016), 3 * Months), make_pair(Date(21, Dec, 2015), Date(20, Mar, 2016)) },
# { make_pair(Date(20, Mar, 2016), 3 * Months), make_pair(Date(21, Dec, 2015), Date(20, Sep, 2016)) },
#
# and produce lines like:
#
# ((Date::new(19, Mar, 2016), Period::new(3, Months)), (Date::new(21, Dec, 2015), Date::new(20, Mar, 2016)) ),
# ((Date::new(20, Mar, 2016), Period::new(3, Months)), (Date::new(21, Dec, 2015), Date::new(20, Sep, 2016)) ),
#
#
cat /tmp/t.txt | awk '{$1=$1;print}' | sed 's/{//g' | sed 's/}//g' | sed 's/make_pair//g' | awk '{$1=$1;print}' | sed -E "s/,$//g" | sed -E "s/Date/Date::new/g" | sed -E "s/([0-9]) \* (Months|Years)/Period::new(\1, \2)/g" | sed -E "s/^\(/((/g" | sed -E "s/$/),/g"
