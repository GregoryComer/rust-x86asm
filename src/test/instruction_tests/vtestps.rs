use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vtestps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 221], OperandSize::Dword)
}

fn vtestps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 970423229, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 132, 218, 189, 123, 215, 57], OperandSize::Dword)
}

fn vtestps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 255], OperandSize::Qword)
}

fn vtestps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 4, 144], OperandSize::Qword)
}

fn vtestps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 246], OperandSize::Dword)
}

fn vtestps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 409473947, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 36, 93, 155, 19, 104, 24], OperandSize::Dword)
}

fn vtestps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 212], OperandSize::Qword)
}

fn vtestps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 20, 83], OperandSize::Qword)
}

