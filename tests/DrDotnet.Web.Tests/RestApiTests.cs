using System.Net;
using System.Text.Json;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.Extensions.DependencyInjection;
using Moq;
using NUnit.Framework;

namespace DrDotnet.Web.Tests;

[NonParallelizable]
public class RestApiTests
{
    [SetUp]
    public void Setup()
    {
        Environment.SetEnvironmentVariable("CONSOLE_LOGGING_ENABLED", "false");
        Environment.SetEnvironmentVariable("FILE_LOGGING_ENABLED", "false");
        Environment.SetEnvironmentVariable("REST_API_ENABLED", "true");
        Environment.SetEnvironmentVariable("WEB_UI_ENABLED", "true");
    }
    
    private HttpClient CreateClient()
    {
        var webApplication = new WebApplicationFactory<Startup>();

        var sessionDiscovery = new Mock<ISessionDiscovery>();
        sessionDiscovery
            .Setup(x => x.GetSessions())
            .Returns(new List<SessionInfo>
            {
                new() { Uuid = "23d65e1f-7522-4317-800c-ef05bfd2f99c", Timestamp = "2023-06-24T14:51:50.833Z" },
                new() { Uuid = "0b456d6d-737d-4775-a353-afc18a317270", Timestamp = "2023-06-15T14:51:59.000Z" }
            });
        
        return webApplication.WithWebHostBuilder(builder => {
            builder.ConfigureServices(services => {
                // Override sessions discovery with mock to avoid relying on actual files
                services.AddSingleton<ISessionDiscovery>(_ => sessionDiscovery.Object);
            });
        }).CreateClient();
    }

    [Test]
    public async Task GetSessions_ListsSessions()
    {
        var httpResponse = await CreateClient().GetAsync("api/sessions");
        var httpContentString = await httpResponse.Content.ReadAsStringAsync();
        
        Assert.That(httpResponse.StatusCode, Is.EqualTo(HttpStatusCode.OK), httpContentString);

        List<SessionInfo>? response = JsonSerializer.Deserialize<List<SessionInfo>>(httpContentString, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
        Assert.NotNull(response);
        
        Assert.True(response!.Any(x => string.Equals(x.Uuid, "23d65e1f-7522-4317-800c-ef05bfd2f99c", StringComparison.OrdinalIgnoreCase)));
    }
    
    [Test]
    public async Task GetProfilers_ListsProfilers()
    {
        var httpResponse = await CreateClient().GetAsync("api/profilers");
        var httpContentString = await httpResponse.Content.ReadAsStringAsync();
        
        Assert.That(httpResponse.StatusCode, Is.EqualTo(HttpStatusCode.OK), $"Expected OK status code but was {httpResponse.StatusCode}. Response: {httpContentString}");

        List<SessionInfo>? response = JsonSerializer.Deserialize<List<SessionInfo>>(httpContentString, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
        Assert.NotNull(response);
        
        Assert.True(response!.Any(x => string.Equals(x.Uuid, "805a308b-061c-47f3-9b30-f785c3186e86", StringComparison.OrdinalIgnoreCase)));
    }
    
    [Test]
    public async Task GetSessions_WhenRestApiIsDisabled_ReturnsNotFound()
    {
        Environment.SetEnvironmentVariable("REST_API_ENABLED", "false");
        
        var httpResponse = await CreateClient().GetAsync("api/sessions");
        var httpContentString = await httpResponse.Content.ReadAsStringAsync();
        
        Assert.That(httpResponse.StatusCode, Is.EqualTo(HttpStatusCode.NotFound), httpContentString);
    }
    
    [Test]
    public void GetSessions_WhenRestApiAndWebUiAreDisabled_ThrowsInvalidOperationException()
    {
        Environment.SetEnvironmentVariable("REST_API_ENABLED", "false");
        Environment.SetEnvironmentVariable("WEB_UI_ENABLED", "false");

        Assert.Throws<InvalidOperationException>(() => CreateClient());
    }
}