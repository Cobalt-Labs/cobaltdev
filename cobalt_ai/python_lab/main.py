import numpy as np
import matplotlib.pyplot as plt
from scratch_model import LinearModel

model = LinearModel()
print(model.predict(5))

# simple dataset
x = np.array([1, 2, 3, 4, 5])
y = np.array([2, 4, 6, 8, 10])

plt.scatter(x, y)
plt.title("Dummy Data")
plt.show()