import numpy as np
from scipy.integrate import quad # type: ignore 

x = [94.0, 205.0, 371.0]
y = [929.0, 902.0, 860.0]

f_interp = lambda xx: np.interp(xx, x, y)

# value at 251.0
print(f_interp(251.0))

# integral from 94.0 to 251.0
print(quad(f_interp, 94.0, 251.0))


def f(t):
    a = 929.0*((t-205.0)*(t-371.0))/((94-205)*(94-371))
    b = 902.0*((t-94.0)*(t-371.0))/((205-94)*(205-371))
    c = 860.0*((t-94.0)*(t-205.0))/((371-94)*(371-205))
    return a + b + c

print(f(251.0))
