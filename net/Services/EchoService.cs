using Grpc.Core;
using GrpcGreeter;
using System.Text;
using System.Security.Cryptography;

namespace GrpcGreeter.Services;

public class EchoService : Echo.EchoBase
{
    private readonly ILogger<EchoService> _logger;
    public EchoService(ILogger<EchoService> logger)
    {
        _logger = logger;
    }

    public override Task<EchoResponse> UnaryEcho(EchoRequest request, ServerCallContext context)
    {
        using (var hashAlgorithm = SHA512.Create())
        {
            var byteValue = Encoding.UTF8.GetBytes(request.Message);
            var byteHash = hashAlgorithm.ComputeHash(byteValue);
            return Task.FromResult(new EchoResponse
            {
                Message = "Echo " + Convert.ToHexString(byteHash)
            });
        }
    }
}
