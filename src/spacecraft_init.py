import numpy as np
import yaml
import numpy.typing as npt



class Spacecraft:
    def __init__(self):
        # Multibody
        self.Jsc = np.array([[10, 0, 0],[0, 20, 0], [0, 0, 30]])

        # Attitude
        self.q_sc_eci = np.array([0, 0, 0, 1])
        self.omega_sc = np.array([0.2, 0.1, 0.3]) * np.pi/180

        # Ephemeris
        a_sc = 6378e3 + 900e3
        mu = 3.986e14
        self.Rsc = np.array([a_sc, 0, 0])
        self.Vsc = np.array([0, np.sqrt(mu/a_sc), 0])

        # Sensors

    def __repr__(self)->str:
        return 'SC at quat @ omega'

def serialize_numpy(obj):
    data = {}
    for key, value in obj.__dict__.items():
        if isinstance(value, np.ndarray):
            if value.ndim == 1:
                data[key] = value.tolist()
            elif value.ndim == 2:
                data[key] = [row.tolist() for row in value]
            else:
                raise ValueError(f"Array {key} has more than 2 dimensions, which is not supported.")
        else:
            # Optionally include non-array fields
            data[key] = value
    return data

def numpy2yaml(arr:npt.NDArray):
    if arr.ndim == 1:
        return [arr.tolist()]
    elif arr.ndim == 2:
        return arr.tolist()
    else:
        raise ValueError(f"Array {arr} has more than 2 dimensions which is not supported")

def to_yaml_dict(sc: Spacecraft):
    return {
        'Sensors': None,
        'Estimation': None,
        'Reference': None,
        'Control': None,
        'Actuators': None,
        'Multibody': {
            'j_multibody': numpy2yaml(sc.Jsc),
        },
        'SC_Actuators': None,
        'SC_Ephemeris': {
            'r_sc': numpy2yaml(sc.Rsc),
            'v_sc': numpy2yaml(sc.Vsc),
        },
        'SC_Attitude': {
            'q_sc_eci': numpy2yaml(sc.q_sc_eci),
            'omega_sc': numpy2yaml(sc.omega_sc),
        },
        'SC_Multibody': {
            'j_multibody': numpy2yaml(sc.Jsc),
        },
        'SC_Sensors': None,

    }

def dump_spacecraft_yaml(sc: Spacecraft, filename="configs/spacecraft.yaml"):
    data = to_yaml_dict(sc)
    with open(filename, 'w') as f:
        yaml.dump(data, f, sort_keys=False)

def main():
    sc = Spacecraft()
    # details = to_yaml_dict(sc)

    a = np.array([1, 2, 3])
    b = np.array([[1, 2, 3],[4, 5, 6]])
    print(numpy2yaml(a))
    print(numpy2yaml(b))

    # print(details)
    dump_spacecraft_yaml(sc)

if __name__ == '__main__':
    main()
