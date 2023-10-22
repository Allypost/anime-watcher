"use client";

import {
  ActionIcon,
  Button,
  type ComboboxData,
  Fieldset,
  Modal,
  NumberInput,
  Select,
  Textarea,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { useMutation } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { type FC, type ReactNode, useMemo } from "react";
import {
  TbEdit as IconEdit,
  TbTrashFilled as IconDelete,
} from "react-icons/tb";

import { type Anime } from "~/api/v1/$models";
import { update as updateAnimeInfo } from "~/api/v1/anime";
import {
  add as addAnimeSource,
  remove as removeAnimeSource,
} from "~/api/v1/animeSources";

const seriesSiteOptions = [
  {
    label: "AniWatch",
    value: "aniwatch",
  },
  {
    label: "AniWave",
    value: "aniwave",
  },
  {
    label: "AllAnime",
    value: "allanime",
  },
] satisfies ComboboxData;

export const AddNewSourceButton: FC<{ forAnimeId: number }> = (props) => {
  const router = useRouter();

  const [isModalOpen, { close: closeModal, open: openModal }] =
    useDisclosure(false);

  type FormValues = {
    seriesSite: string;
    seriesSiteId: string;
  };

  const addSourceMutation = useMutation({
    mutationFn: async (values: FormValues) => {
      return addAnimeSource({
        seriesId: props.forAnimeId,
        seriesSite: values.seriesSite,
        seriesSiteId: values.seriesSiteId,
      });
    },
  });

  const form = useForm<FormValues>({
    initialValues: {
      seriesSite: "",
      seriesSiteId: "",
    },

    validate: {
      seriesSiteId: (value) => {
        value = value.trim();

        if (!value) {
          return "Required";
        }

        if (value.length < 3) {
          return "Too short";
        }

        return undefined;
      },
    },
  });

  const isLoading = addSourceMutation.isPending;

  return (
    <>
      <Modal
        centered
        closeOnClickOutside={!addSourceMutation.isPending}
        closeOnEscape={!addSourceMutation.isPending}
        opened={isModalOpen}
        size="lg"
        title="Add new source"
        overlayProps={{
          blur: 3,
        }}
        onClose={closeModal}
      >
        <form
          className="relative flex flex-col gap-4"
          onReset={(...args) => {
            form.onReset(...args);
          }}
          onSubmit={form.onSubmit((values) => {
            addSourceMutation.mutate(values, {
              onSuccess: (data) => {
                const body = data?.body;

                switch (body?.type) {
                  case undefined:
                  case "empty": {
                    return notifications.show({
                      title: "Error",
                      message: "Got empty response",
                      color: "red",
                    });
                  }

                  case "error": {
                    return notifications.show({
                      title: "Error",
                      message: body.data,
                      color: "red",
                    });
                  }

                  case "success": {
                    form.reset();
                    closeModal();
                    router.refresh();

                    return notifications.show({
                      title: "Success",
                      message: "Source added",
                      color: "green",
                    });
                  }

                  default: {
                    // Check exhaustiveness
                    body satisfies never;
                  }
                }
              },
            });
          })}
        >
          <Fieldset
            className="flex flex-col gap-[inherit]"
            disabled={isLoading}
          >
            <Select
              required
              searchable
              data={seriesSiteOptions}
              label="Series site"
              placeholder="AniWatch"
              withScrollArea={false}
              {...form.getInputProps("seriesSite")}
            />

            <TextInput
              required
              autoComplete="off"
              label="ID on series site"
              placeholder="the-eminence-in-shadow-season-2-18505"
              {...form.getInputProps("seriesSiteId")}
            />
          </Fieldset>

          <div className="flex">
            <Button
              color="yellow"
              size="sm"
              type="reset"
              variant="outline"
              onClick={(e) => {
                e.preventDefault();

                form.reset();
              }}
            >
              Clear
            </Button>

            <Button
              className="ml-auto"
              color="green"
              size="sm"
              type="submit"
              variant="outline"
            >
              Add
            </Button>
          </div>
        </form>
      </Modal>
      <Button
        color="white"
        size="compact-md"
        variant="subtle"
        onClick={openModal}
      >
        Add new source
      </Button>
    </>
  );
};

export const DeleteSourceButton: FC<{
  sourceId: number;
  confirmText: ReactNode;
}> = ({ sourceId, confirmText }) => {
  const router = useRouter();

  const removeSourceMutation = useMutation({
    mutationFn: async () => {
      const notifId = notifications.show({
        title: "Deleting source",
        message: "Deleting source...",
        loading: true,
        autoClose: false,
      });

      const resp = await removeAnimeSource(sourceId);

      notifications.update({
        id: notifId,
        loading: false,
        message: "",
        withCloseButton: true,
        autoClose: true,
      });

      const body = resp?.body;
      switch (body?.type) {
        case undefined:
        case "empty": {
          return notifications.update({
            id: notifId,
            message: "Got empty response",
            color: "red",
          });
        }

        case "error": {
          return notifications.update({
            id: notifId,
            message: body.data,
            color: "red",
          });
        }

        case "success": {
          router.refresh();

          return notifications.update({
            id: notifId,
            message: "Source deleted",
            color: "green",
          });
        }

        default: {
          // Check exhaustiveness
          body satisfies never;
        }
      }

      notifications.update({
        id: notifId,
        message: "Not implemented",
      });
    },
  });

  const isLoading = removeSourceMutation.isPending;

  return (
    <ActionIcon
      aria-label="Delete source"
      color="red"
      data-id={sourceId}
      disabled={isLoading}
      radius="xl"
      size="lg"
      type="button"
      variant="subtle"
      onClick={(e) => {
        e.preventDefault();

        modals.openConfirmModal({
          title: "Delete source",
          children: confirmText,
          centered: true,
          labels: {
            cancel: "Cancel",
            confirm: "Delete",
          },
          onConfirm: () => {
            removeSourceMutation.mutate();
          },
        });
      }}
    >
      <IconDelete className="h-full w-full p-1" />
    </ActionIcon>
  );
};

export const EditAnimeInfoButton: FC<{
  anime: Anime;
}> = (props) => {
  const router = useRouter();
  const [isModalOpen, { close: closeModalFn, open: openModal }] =
    useDisclosure(false);

  type FormValues = {
    name: string;
    description: string | null;
    malId: number | null;
  };

  const form = useForm<FormValues>({
    initialValues: {
      ...props.anime,
    },

    validate: {
      name: (value) => {
        value = value.trim();

        if (!value) {
          return "Required";
        }

        if (value.length < 3) {
          return "Too short";
        }

        return undefined;
      },
      malId: (value) => {
        if (value === null) {
          return undefined;
        }

        if (value <= 0) {
          return "Invalid ID";
        }

        return undefined;
      },
    },

    clearInputErrorOnChange: true,
    validateInputOnBlur: true,
  });

  const closeModal = () => {
    closeModalFn();
    form.reset();
  };

  const updateAnimeMutation = useMutation({
    mutationFn: async (values: FormValues) => {
      return updateAnimeInfo(props.anime.id, values);
    },
  });

  const isLoading = updateAnimeMutation.isPending;

  return (
    <>
      <Modal
        centered
        closeOnClickOutside={!updateAnimeMutation.isPending}
        closeOnEscape={!updateAnimeMutation.isPending}
        opened={isModalOpen}
        size="lg"
        title="Edit anime info"
        overlayProps={{
          blur: 3,
        }}
        onClose={closeModal}
      >
        <form
          className="relative flex flex-col gap-4"
          onReset={(...args) => {
            form.onReset(...args);
          }}
          onSubmit={form.onSubmit((values) => {
            updateAnimeMutation.mutate(values, {
              onSuccess: (data) => {
                const body = data?.body;

                switch (body?.type) {
                  case undefined:
                  case "empty": {
                    return notifications.show({
                      title: "Error",
                      message: "Got empty response",
                      color: "red",
                    });
                  }

                  case "error": {
                    return notifications.show({
                      title: "Error",
                      message: body.data,
                      color: "red",
                    });
                  }

                  case "success": {
                    router.refresh();
                    closeModal();

                    return notifications.show({
                      title: "Success",
                      message: "Anime edited",
                      color: "green",
                    });
                  }

                  default: {
                    // Check exhaustiveness
                    body satisfies never;
                  }
                }
              },
            });
          })}
        >
          <Fieldset
            className="flex flex-col gap-[inherit]"
            disabled={isLoading}
          >
            <TextInput
              required
              label="Anime name"
              placeholder="The Eminence in Shadow Season 2"
              {...form.getInputProps("name")}
            />

            <Textarea
              autosize
              label="Description"
              maxRows={20}
              {...form.getInputProps("description")}
            />

            <NumberInput
              label="MyAnimeList ID"
              min={0}
              placeholder="12345"
              {...form.getInputProps("malId")}
            />
          </Fieldset>

          <div className="flex">
            <Button
              color="yellow"
              disabled={isLoading}
              size="sm"
              type="reset"
              variant="outline"
              onClick={(e) => {
                e.preventDefault();

                form.reset();
              }}
            >
              Reset
            </Button>

            <Button
              className="ml-auto"
              color="green"
              loading={isLoading}
              size="sm"
              type="submit"
              variant="outline"
            >
              Save
            </Button>
          </div>
        </form>
      </Modal>
      <Button
        color="white"
        leftSection={<IconEdit className="h-full w-full" />}
        size="lg"
        variant="subtle"
        onClick={() => {
          form.reset();
          openModal();
        }}
      >
        Edit
      </Button>
    </>
  );
};
