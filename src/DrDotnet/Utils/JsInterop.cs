using System.IO;
using System.IO.Compression;
using System.Threading.Tasks;
using Microsoft.JSInterop;

namespace DrDotnet.Utils;

public static class JsInterop
{
    public static async Task DownloadZipAsync(this SessionInfo session, IJSRuntime jsRuntime)
    {
        var memoryStream = new MemoryStream();

        using var archive = new ZipArchive(memoryStream, ZipArchiveMode.Create, leaveOpen: true);
      
        foreach (var file in session.EnumerateReports())
        {
            archive.CreateEntryFromFile(file.FullName, file.Name);
        }
            
        memoryStream.Seek(0, SeekOrigin.Begin);
        
        using var streamRef = new DotNetStreamReference(stream: memoryStream, leaveOpen: true);

        await jsRuntime.InvokeVoidAsync("downloadFileFromStream", $"session-{session.Guid}.zip", streamRef);
    }
}