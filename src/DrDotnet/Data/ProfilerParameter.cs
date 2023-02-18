
using System.Globalization;

public partial class ProfilerParameter
{
    public int ValueInt32
    {
        get => int.Parse(Value);
        set => Value = value.ToString();
    }
    
    public float ValueFloat32
    {
        get => float.Parse(Value, CultureInfo.InvariantCulture);
        set => Value = value.ToString(CultureInfo.InvariantCulture);
    }
    
    public bool ValueBoolean
    {
        get => bool.Parse(Value);
        set => Value = value.ToString();
    }
}