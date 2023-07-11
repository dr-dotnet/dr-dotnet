
using System.Globalization;

public partial class ProfilerParameter
{
    public int ValueInt32
    {
        get => int.TryParse(Value, out int value) ? value : 0;
        set => Value = value.ToString();
    }
    
    public float ValueFloat32
    {
        get => float.TryParse(Value, CultureInfo.InvariantCulture, out float value) ? value : 0;
        set => Value = value.ToString(CultureInfo.InvariantCulture);
    }
    
    public bool ValueBoolean
    {
        get => bool.TryParse(Value, out bool value) && value;
        set => Value = value.ToString();
    }
}