from enum import Enum

class ResponseCode(Enum):
    NOERROR = 0
    FORMATERR = 1
    SERVFAIL = 2  # rare chance for this to happen on any valid request
    NXDOMAIN = 3
    REFUSED = 5  # rare chance for this to be answer for an IP that isn't a NS server

class NsQuestionPart:
    """
    This represents a record in the question section.
    """
    def __init__(self):
        self.qname = ''
        self.qtype = 'IN' # Always internet
        self.qclass = ''

class NsResourcePart:
    """
    This represents a resource record in the answer or NS or additional section.
    """

    def __init__(self):
        self.name = ''
        self.type = 'IN'  # always internet
        self.qclass = ''
        self.ttl = 0
        self.rdata = ''

class NsResponse:
    """
    This class represents a response from a DNS server.
    """

    def __init__(self):
        self.id = 1337  # TODO: randomize this
        self.opcode = 0  # standard query -- must always be this
        self.is_authority = False
        self.truncation = False  # should be always unset
        self.recursion_desired = True  # the client always wants a recursive answer...
        self.recursion_available = False  # ...but in this world, recursive servers don't exist :(
        
        self.questions = []
        self.answers = []
        self.authorities = []
        self.additionals = []
