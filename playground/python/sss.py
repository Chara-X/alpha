# -*- coding: utf-8 -*-

import sys

from commons.common_consts.common_file_const import CommonFileConst
from commons.common_utils.common_util import CommonUtil
from commons.common_exceptions.common_exceptions import (
    CheckParamsException,
    PidException,
)
from commons.common_utils.common_logger import CommonLogger
from service.config_service.check_params_service import CheckParamsService
from controller.deploy_controller import DeployController
from commons.common_utils.common_sub_util import CommonSubUtil
from commons.common_consts.common_const import CommonConst


class DeployMentProcess(object):
    def __init__(self):
        self.__act = None
        self.__prod = None
        self.__additional_params = dict()

    def check_params(self, args):
        try:
            check_params_service = CheckParamsService(args)
            check_params_service.check()
            self.__act = check_params_service.get_action()
            self.__prod = check_params_service.get_product()
            self.__additional_params.update(
                check_params_service.get_additional_params()
            )
        except CheckParamsException as result:
            raise Exception(result)
        return self

    def init(self):
        try:
            self.__init_deploy_pid().__init_logger()
        except PidException:
            self.__print_err_msg("initial deployment failed.")
            raise
        except Exception:
            self.__print_err_msg("initial deployment failed.")
            raise
        else:
            return self

    def deploy_process(self):
        DeployController(
            act=self.__act, prod=self.__prod, additional_param=self.__additional_params
        ).deploy()
        return self

    @classmethod
    def finish_handle(cls):
        cls.clearAll()

    def __init_deploy_pid(self):
        if CommonUtil.is_exists(CommonFileConst.PID_FILE):
            raise PidException("deploy process is running ...")
        else:
            self.__print_msg("create deploy pid ...")
            if not CommonUtil.is_exists(CommonFileConst.DEPLOY_LOG_DIR):
                CommonUtil.make_dirs(CommonFileConst.DEPLOY_LOG_DIR)
            CommonUtil.create_file(CommonFileConst.PID_FILE)
        return self

    def __init_logger(self):
        try:
            CommonLogger(act=self.__act)
        except Exception as e:
            raise Exception("init logger failed. \n[{}]".format(e))

    @classmethod
    def clearAll(cls):
        if CommonUtil.is_exists(CommonFileConst.PID_FILE):
            cls.__print_msg("clear deploy pid.")
            CommonUtil.delete_file(CommonFileConst.PID_FILE)

    @staticmethod
    def __print_msg(msgs):
        print(msgs)

    @staticmethod
    def __print_err_msg(msgs):
        print("[ERROR] --> {}".format(msgs))

    @staticmethod
    def _update_action_result_failed(args):
        if len(args) > CommonConst.MIN_DEPLOY_PARAMS_NUM:
            if CommonUtil.is_exists_key_in_ini(
                CommonFileConst.LOCAL_VTASK_CONFIG_INI, args[1], args[2]
            ):
                CommonSubUtil.update_vtask_config(
                    CommonFileConst.LOCAL_VTASK_CONFIG_INI,
                    args[1],
                    args[2],
                    CommonConst.RECORD_FAILED,
                )

    @staticmethod
    def get_deploy_log_folder_name(args):
        if len(args) > CommonConst.MIN_DEPLOY_PARAMS_NUM:
            return args[1] + "_" + args[2]
        if len(args) == CommonConst.MIN_DEPLOY_PARAMS_NUM:
            return args[1]
        return "install"


# Started by AICoder, pid:we9aav4e4budf5714c7308c810116021c3c450d5
def main(args):
    log_folder_name = DeployMentProcess.get_deploy_log_folder_name(args)
    CUR_LOGGER = CommonLogger(log_folder_name, act=log_folder_name)
    try:
        DeployMentProcess().check_params(args).init().deploy_process().finish_handle()
    except KeyboardInterrupt:
        DeployMentProcess.clearAll()
        DeployMentProcess._update_action_result_failed(args)
        print("{} director was interrupted.".format(args[1]))
        CUR_LOGGER.error("{} director was interrupted.".format(args[1]))
        sys.exit(1)
    except PidException as res:
        DeployMentProcess._update_action_result_failed(args)
        print("{0} director failed. \n{1}".format(args[1], res))
        CUR_LOGGER.error("{0} director failed. \n{1}".format(args[1], res))
        sys.exit(1)
    except Exception as res:
        DeployMentProcess.clearAll()
        DeployMentProcess._update_action_result_failed(args)
        print("{0} director failed. \n{1}".format(args[1], res))
        CUR_LOGGER.error("{0} director failed. \n{1}".format(args[1], res))
        sys.exit(1)
    else:
        CUR_LOGGER.info("deploy director success.")
        print("deploy director success.")
        sys.exit(0)


if __name__ == "__main__":
    main(sys.argv)
# Ended by AICoder, pid:we9aav4e4budf5714c7308c810116021c3c450d5
