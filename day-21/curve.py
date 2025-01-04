nix-shell -p  "python3.withPackages(ps: [ps.ipython ps.numpy ps.scipy ps.matplotlib ps.pandas])"

import numpy as np
import io
import pandas as pd
from scipy.optimize import curve_fit
from scipy.interpolate import lagrange
import subprocess

# Command to call the external program
command = ["cargo", "run", "--release"]

# Run the program and capture its output
result = subprocess.run(command, stdout=subprocess.PIPE, text=True, check=True)

data = pd.read_csv(io.StringIO(result.stdout), delimiter=';')

# Replace 'data.csv' with the path to your CSV file
steps = data['x'].values  # Independent variable (x)
plots = data['y'].values  # Dependent variable (y)

# 2. Define the model functions

# Quadratic model (ax^2 + bx + c)
def quadratic(x, a, b, c):
    return a * x**2 + b * x + c

# Power law model (a * x^b)
def power_law(x, a, b):
    return a * x**b

# 3. Perform curve fitting for each model

# Quadratic fit
params_quadratic, _ = curve_fit(quadratic, steps, plots)

# Power law fit
params_power_law, _ = curve_fit(power_law, steps, plots, p0=(1, 1))

# 4. Perform Lagrangian interpolation
lagrange_poly = lagrange(steps, plots)

x = 982
x = 26501365

# 65 196 327 458 589 720 851 982 1113 1244 1375 1506 1637 1768 1899 2030 2161 2292 2423 2554

# 5. Predict the value for 5000 steps using all models
predicted_quadratic = quadratic(x, *params_quadratic)
# predicted_power_law = power_law(5000, *params_power_law)
predicted_lagrange = lagrange_poly(x)

# 6. Print predicted values
print(f"Predicted garden plots for {x} steps:")
print(f"Quadratic: {predicted_quadratic:.2f}")
print(f"Lagrangian Interpolation: {predicted_lagrange:.2f}")

