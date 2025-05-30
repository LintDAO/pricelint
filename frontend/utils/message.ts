import { Notify } from "quasar"

export const showMessageError = (message: string) =>
  Notify.create({
    message,
    type: "negative",
    progress: true,
    position: "top",
  })
export const showResultError = (res: any) => {
  let message
  if (res.Err) {
    message = "Error: " + res.Err
  } else {
    //一般来说只有res.Err才会调用这个方法，多写个else防止意外
    message = JSON.stringify(res)
  }
  Notify.create({
    message,
    type: "negative",
    progress: true,
    position: "top",
  })
}
export const showMessageSuccess = (message: string) =>
  Notify.create({
    message,
    type: "positive",
    progress: true,
    position: "top",
  })
