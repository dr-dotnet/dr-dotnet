using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;

namespace DrDotnet.Web;

public class Program
{
    public static void Main(string[] args)
    {
        CreateHostBuilder(args).Build().Run();
    }

    private static IHostBuilder CreateHostBuilder(string[] args)
    {
        return Host.CreateDefaultBuilder(args)
            .ConfigureWebHostDefaults(webBuilder =>
            {
                webBuilder.UseStartup<Startup>();
                webBuilder.UseUrls(@"http://*:92");
                webBuilder.UseSetting(WebHostDefaults.DetailedErrorsKey, "true");
            });
    }
}