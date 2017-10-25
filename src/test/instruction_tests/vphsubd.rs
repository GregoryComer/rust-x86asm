use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 6, 200], OperandSize::Dword)
}

fn vphsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 6, 10], OperandSize::Dword)
}

fn vphsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 6, 196], OperandSize::Qword)
}

fn vphsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 6, 41], OperandSize::Qword)
}

fn vphsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 6, 240], OperandSize::Dword)
}

fn vphsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 1003784701, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 6, 146, 253, 137, 212, 59], OperandSize::Dword)
}

fn vphsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 6, 208], OperandSize::Qword)
}

fn vphsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 6, 12, 240], OperandSize::Qword)
}

