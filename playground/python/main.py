import copy
import sys

from commons.common_consts.common_const import CommonConst
from commons.common_utils.common_util import CommonUtil
from commons.common_consts.common_file_const import CommonFileConst

from service.deploy_service.base_deploy_service import BaseDeployService


class UpgradeService(BaseDeployService):
    def __init__(self, act="upgrade", prod=None, params=None):
        super(UpgradeService, self).__init__(act=act, prod=prod, params=params)
        self.__act = act
        self.__prod = prod
        self.__params = params
        self.__cmd_list = []

    def get_cmd_list(self):
        return self.__cmd_list

    def clean_before_upgrade(self):
        scene = self.basic_conf.get_deploy_config().get_scene()
        self.config_service.clean_kafka_info(scene)
        return self

    def tag_gluster_labels(self):
        from service.config_service.node_labels_service import NodeLabelsService

        NodeLabelsService().tag_gluster_server_label()
        return self

    def init_information(self):
        self.init_service_info().init_info_from_paas().init_envs().convert_configurations()
        self.set_tenant_pkg_id()
        self.logger.info("Init information before deploy complete.")

    def init_service_info(self):
        self.init_configuration_files().init_basic_files().init_act().init_services()
        return self

    def init_configuration_files(self):
        self.init_vnm_network().init_paas_conf().init_deploy_config().init_cassandra_conf().init_config_conf().init_conf_json().init_product_config()
        return self

    def init_info_from_paas(self):
        if self.init_base_info_from_paas() and self.config_service.config_cpaas_info():
            return self
        else:
            raise Exception("init basic information from pass failed.")

    def construct_cmd(self):
        self.set_version_type()
        self.__set_tenant()
        self.__build_mbr_cmd()
        self.conf_slb_mem()
        self.__build_command()
        # self.set_base_cmd(cmd)

    def __set_tenant(self):
        tenant_params = self.__params.get("tenant")
        if tenant_params:
            self.set_tenant(tenant_params)

    def __get_web_hook_cmd(self):
        result = self.build_web_hook_cmd()
        if result:
            self.__cmd_list.append(result)

    def __get_dexcloud_cmd(self, ori_cmd):
        result = self.build_dexcloud_command(ori_cmd)
        if result != "":
            self.__cmd_list.append(result)

    def __build_command(self):
        if self.is_product():
            return True
        ori_cmd = self.get_base_cmd()
        resource_files = ",".join(self.get_resource_files())
        if self.__is_pkg_upgrade():
            cmd = self.__build_pkg_upgrade_command(ori_cmd, resource_files)
        elif self.__is_prepare_app_images():
            self.__get_dexcloud_cmd(ori_cmd)
            additional_param = "--step {}".format(CommonConst.OKI_PREPARE_APP_CMD)
            cmd = self.__build_oki_upgrade_cmd(
                ori_cmd, resource_files, additional_params=additional_param
            )
        elif self.__is_upgrade_prepared_apps():
            self.__get_web_hook_cmd()
            additional_param = "--step {}".format(
                CommonConst.OKI_UPGRADE_PREPARED_APP_CMD
            )
            cmd = self.__build_oki_upgrade_cmd(
                ori_cmd, resource_files, additional_params=additional_param
            )
        else:
            self.__get_web_hook_cmd()
            self.__get_dexcloud_cmd(ori_cmd)
            additional_param = self.build_same_version_param()
            cmd = self.__build_oki_upgrade_cmd(
                ori_cmd, resource_files, additional_params=additional_param
            )
        self.__cmd_list.append(cmd)
        return cmd

    def __is_pkg_upgrade(self):
        """是否分组包升级"""
        return "package" == self.__prod

    def __is_prepare_app_images(self):
        return self.__prod == CommonConst.PREPARE_UPGRADE

    def __is_upgrade_prepared_apps(self):
        return self.__prod == CommonConst.PREPARED_APPS

    def __build_pkg_upgrade_command(self, ori_cmd, resource_file):
        target_list = self.__get_target_list()
        to_upgrade_pkg = self.__filter_upgrade_pkgs(target_list)
        return "{0} package -r {1} --target {2} --dir {3}".format(
            ori_cmd, resource_file, to_upgrade_pkg[0], CommonFileConst.VERSION_DIR
        )

    def __build_oki_upgrade_cmd(self, cmd, res_file, additional_params=""):
        scene_file = self.get_scene_files()
        return "{0} --resource {1} --scene {2} --dir {3} {4}".format(
            cmd, res_file, scene_file, CommonFileConst.VERSION_DIR, additional_params
        ).strip()

    def __get_target_list(self):
        target = self.__params.get("target")
        target_list = target.split(",") if "," in target else [target]
        return target_list

    def __filter_upgrade_pkgs(self, target_list):
        no_exist_list = copy.deepcopy(target_list)
        to_upgrade_pkg = []
        for source_name in CommonUtil.list_items_in_dir(
            CommonFileConst.VERSION_SOURCES_DIR
        ):
            self.__filter_sources_group_pkg(
                target_list, no_exist_list, to_upgrade_pkg, source_name
            )
        if len(no_exist_list) > 0:  # nosec
            self.logger.warn("the packages {} do not exist.".format(no_exist_list))
        return to_upgrade_pkg

    def __filter_sources_group_pkg(
        self, target_list, no_exist_list, to_upgrade_pkg, source_name
    ):
        self.logger.info("check {} group packages ...".format(source_name))
        source_path = CommonUtil.get_combined_path(
            CommonFileConst.VERSION_SOURCES_DIR, source_name
        )
        director_group_pkg_dirs = CommonUtil.list_items_in_dir(source_path)
        for pkg_dir in director_group_pkg_dirs:
            for item in target_list:
                if item.lower() != pkg_dir.lower():
                    continue
                if item in no_exist_list:
                    to_upgrade_pkg.append(
                        CommonUtil.get_combined_path(source_path, pkg_dir)
                    )
                    no_exist_list.remove(item)
        return self

    def execute_command(self):
        self.exec_amd(self.__cmd_list)

    def conf_slb_mem(self):
        if self.basic_conf.get_slb_num() == 0:
            self.logger.info("SLB not deployed, not conf with SLB memory")
            return
        mem_limit = self.basic_conf.get_vnm_network().get_slb_mem_limit()
        if mem_limit < CommonConst.SLB_MEM_LIMIT:
            cmd = "{} {} {} {} > {} && chmod 640 {}".format(
                sys.executable,
                CommonFileConst.SLB_ROUTE_LIMIT_FILE,
                CommonConst.SLB_CPU_LIMIT,
                CommonConst.SLB_MEM_LIMIT,
                CommonFileConst.SLB_MEM_LOG,
                CommonFileConst.SLB_MEM_LOG,
            )
            self.__cmd_list.append(cmd)

    def __build_mbr_cmd(self):
        if (
            self.basic_conf.get_vnm_network().has_gr_label()
            and not self.__is_pkg_upgrade()
            and not self.is_product()
        ):
            ori_cmd = "sh {} update > {} && chmod 640 {}".format(
                CommonFileConst.MBR_SH, CommonFileConst.MBR_LOG, CommonFileConst.MBR_LOG
            )
            self.__cmd_list.append(ori_cmd)

    def complete(self):
        if self.__is_prepare_app_images():
            self.logger.info("Prepare app images complete.")
            return True
        self.logger.info("Upgrade director complete. Start to deal with aftermath.")
        self.deal_after_upgrade()
        self.logger.info("{0} director success.".format(self.__act))
        return True

    def deal_after_upgrade(self):
        self.sync_files().__open_ports()

    def __open_ports(self):
        to_del_ports = "1161,35000,40001,40041"
        self.config_service.operate_ports("open", ports=to_del_ports)
        return self
