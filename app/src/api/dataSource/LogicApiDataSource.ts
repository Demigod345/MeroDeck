import {
  ApiResponse,
  JsonRpcClient,
  RequestConfig,
  WsSubscriptionsClient,
  RpcError,
  handleRpcError,
  RpcQueryParams,
} from '@calimero-is-near/calimero-p2p-sdk';
import {
  ApproveProposalRequest,
  ApproveProposalResponse,
  ClientApi,
  ClientMethod,
  CreateProposalRequest,
  CreatePlayerChangeRequest,
  CreatePlayerChangeResponse,
  CreateProposalResponse,
  GetProposalMessagesRequest,
  GetProposalMessagesResponse,
  GetActivePlayerResponse,
  SendProposalMessageRequest,
  SendProposalMessageResponse,
  GetActivePlayerRequest,
  GetGameStateRequest,
  GetGameStateResponse,
  CreateActionRequest,
  CreateActionResponse,
  JoinGameRequest,
  JoinGameResponse,
} from '../clientApi';
import { getContextId, getNodeUrl } from '../../utils/node';
import {
  getJWTObject,
  getStorageAppEndpointKey,
  JsonWebToken,
} from '../../utils/storage';
import { AxiosHeader, createJwtHeader } from '../../utils/jwtHeaders';
import { getRpcPath } from '../../utils/env';
import GameEventListener from '../../utils/GameEventListener';

export function getJsonRpcClient() {
  return new JsonRpcClient(getStorageAppEndpointKey() ?? '', getRpcPath());
}

export function getWsSubscriptionsClient() {
  return new WsSubscriptionsClient(getStorageAppEndpointKey() ?? '', '/ws');
}

export function getGameEventListener() {
  return new GameEventListener(getNodeUrl(), getStorageAppEndpointKey() ?? '');
}

export function getConfigAndJwt() {
  const jwtObject: JsonWebToken | null = getJWTObject();
  const headers: AxiosHeader | null = createJwtHeader();
  if (!headers) {
    return {
      error: { message: 'Failed to create auth headers', code: 500 },
    };
  }
  if (!jwtObject) {
    return {
      error: { message: 'Failed to get JWT token', code: 500 },
    };
  }
  if (jwtObject.executor_public_key === null) {
    return {
      error: { message: 'Failed to get executor public key', code: 500 },
    };
  }

  const config: RequestConfig = {
    headers: headers,
    timeout: 10000,
  };

  return { jwtObject, config };
}

export class LogicApiDataSource implements ClientApi {


  async createAction(
    request: CreateActionRequest,
  ): ApiResponse<CreateActionResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    console.log('Creating action with request:', request);

    const params: RpcQueryParams<typeof request> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.CREATE_ACTION,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    };

    console.log('RPC params:', params);

    const response = await getJsonRpcClient().execute<
      typeof request,
      CreateActionResponse
    >(params, config);

    console.log('Raw response:', response);

    if (response?.error) {
      console.error('RPC error:', response.error);
      return await this.handleError(response.error, {}, this.createAction);
    }

    return {
      data: response.result.output as CreateActionResponse,
      error: null,
    };
  }


  async joinGame(
    request: JoinGameRequest,
  ): ApiResponse<JoinGameResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    const params: RpcQueryParams<JoinGameRequest> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.JOIN_GAME,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    };

    console.log('RPC params:', params);

    const response = await getJsonRpcClient().execute<
      JoinGameRequest,
      JoinGameResponse
    >(params, config);

    console.log('Raw response:', response);

    if (response?.error) {
      console.error('RPC error:', response.error);
      return await this.handleError(response.error, {}, this.joinGame);
    }

    return {
      data: response.result.output as JoinGameResponse,
      error: null,
    };
  }

  

  async changePlayer(
    request: CreatePlayerChangeRequest,
  ): ApiResponse<CreatePlayerChangeResponse> {
    const { jwtObject, config, error } = getConfigAndJwt(); // This JWT token stores the contextId and executorPublicKey
    if (error) {
      return { error };
    }

    console.log('Creating request to change the current player:', request);

    const params: RpcQueryParams<typeof request> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.SET_ACTIVE_PLAYER,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    };

    console.log('RPC params:', params);

    const response = await getJsonRpcClient().execute<
        typeof request,
        CreatePlayerChangeResponse
      >(params, config);

    console.log('Raw response:', response);

    if (response?.error) {
      console.error('RPC error:', response.error);
      return await this.handleError(response.error, {}, this.changePlayer);
    }

    return {
      data: {},
      error: null,
    };
  }




  async createProposal(
    request: CreateProposalRequest,
  ): ApiResponse<CreateProposalResponse> {
    const { jwtObject, config, error } = getConfigAndJwt(); // This JWT token stores the contextId and executorPublicKey
    if (error) {
      return { error };
    }

    console.log('Creating proposal with request:', request);

    const params: RpcQueryParams<typeof request> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.CREATE_PROPOSAL,
      argsJson: {
        request: request,
      },
      executorPublicKey: jwtObject.executor_public_key,
    };

    console.log('RPC params:', params);

    try {
      const response = await getJsonRpcClient().execute<
        typeof request,
        CreateProposalResponse
      >(params, config);

      console.log('Raw response:', response);

      if (response?.error) {
        console.error('RPC error:', response.error);
        return await this.handleError(response.error, {}, this.createProposal);
      }

      if (!response?.result?.output) {
        console.error('Invalid response format:', response);
        return {
          error: { message: 'Invalid response format', code: 500 },
          data: null,
        };
      }

      return {
        data: response.result.output as CreateProposalResponse,
        error: null,
      };
    } catch (err) {
      console.error('Unexpected error:', err);
      return {
        error: { message: err.message || 'Unexpected error', code: 500 },
        data: null,
      };
    }
  }

  async approveProposal(
    request: ApproveProposalRequest,
  ): ApiResponse<ApproveProposalResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    console.log('appoveProposal', request);

    const params: RpcQueryParams<ApproveProposalRequest> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.APPROVE_PROPOSAL,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    };

    const response = await getJsonRpcClient().execute<
      ApproveProposalRequest,
      ApproveProposalResponse
    >(params, config);

    console.log('appoveProposal response', response);

    if (response?.error) {
      return await this.handleError(response.error, {}, this.approveProposal);
    }

    return {
      data: {},
      error: null,
    };
  }

  async getProposalMessages(
    request: GetProposalMessagesRequest,
  ): ApiResponse<GetProposalMessagesResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    console.log('getProposalMessages', request);

    const params: RpcQueryParams<GetProposalMessagesRequest> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.GET_PROPOSAL_MESSAGES,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    };

    const response = await getJsonRpcClient().query<
      GetProposalMessagesRequest,
      GetProposalMessagesResponse
    >(params, config);

    console.log('getProposalMessages response', response);

    if (response?.error) {
      return await this.handleError(
        response.error,
        {},
        this.getProposalMessages,
      );
    }

    let getProposalsResponse: GetProposalMessagesResponse = {
      messages: response?.result?.output?.messages,
    } as GetProposalMessagesResponse;

    return {
      data: getProposalsResponse,
      error: null,
    };
  }

  async getActivePlayer(
    request: GetActivePlayerRequest,
  ): ApiResponse<GetActivePlayerResponse>{
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    const params: RpcQueryParams<GetActivePlayerRequest> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.GET_ACTIVE_PLAYER,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    }

    const response = await getJsonRpcClient().query<
      GetActivePlayerRequest,
      GetActivePlayerResponse
    >(params, config);

    console.log('Response recieved from logicapi', response);

    if (response?.error) {
      return await this.handleError(
        response.error,
        {},
        this.getActivePlayer,
      );
    }

    let getActivePlayerResponse: GetActivePlayerResponse = {
      active_player: response?.result?.output,
    } as GetActivePlayerResponse;

    return {
      data: getActivePlayerResponse,
      error: null
    }

  }

  async getGameState(
    request: GetGameStateRequest,
  ): ApiResponse<GetGameStateResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    const params: RpcQueryParams<GetGameStateRequest> = {
      contextId: jwtObject?.context_id ?? getContextId(),
      method: ClientMethod.GET_GAME_STATE,
      argsJson: request,
      executorPublicKey: jwtObject.executor_public_key,
    }

    const response = await getJsonRpcClient().query<
      GetGameStateRequest,
      GetGameStateResponse
    >(params, config);

    // console.log('Response recieved from logicapi', response);

    if (response?.error) {
      return await this.handleError(
        response.error,
        {},
        this.getGameState,
      );
    }

    let getGameStateResponse: GetGameStateResponse = {
      game_state: response?.result?.output,
    } as GetGameStateResponse;

    return {
      data: getGameStateResponse,
      error: null
    }
  }


  async sendProposalMessage(
    request: SendProposalMessageRequest,
  ): ApiResponse<SendProposalMessageResponse> {
    const { jwtObject, config, error } = getConfigAndJwt();
    if (error) {
      return { error };
    }

    const response = await getJsonRpcClient().execute<
      SendProposalMessageRequest,
      SendProposalMessageResponse
    >(
      {
        contextId: jwtObject?.context_id ?? getContextId(),
        method: ClientMethod.SEND_PROPOSAL_MESSAGE,
        argsJson: request,
        executorPublicKey: jwtObject.executor_public_key,
      },
      config,
    );
    if (response?.error) {
      return await this.handleError(
        response.error,
        {},
        this.sendProposalMessage,
      );
    }

    return {
      data: {},
      error: null,
    };
  }

  private async handleError(
    error: RpcError,
    params: any,
    callbackFunction: any,
  ) {
    if (error && error.code) {
      const response = await handleRpcError(error, getNodeUrl);
      if (response.code === 403) {
        return await callbackFunction(params);
      }
      return {
        error: await handleRpcError(error, getNodeUrl),
      };
    }
  }
}
