using System;
using System.Linq;

public partial class ProfilerInfo
{
    public Guid Guid => new(Uuid);

    public void SetParameter<T>(string name, T value)
    {
        Parameters.First(x => x.Key == name).Value = value!.ToString();
    }
}