using System.IO;

namespace DrDotnet.Utils;

public static class PathUtils
{
    public static string DrDotnetBaseDirectory {
        get {
            var dir = Path.Combine(Path.GetTempPath(), "dr-dotnet");
            if (!Directory.Exists(dir)) {
                Directory.CreateDirectory(dir);
            }
            return dir;
        }
    }
}
