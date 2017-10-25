use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vhaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 124, 212], OperandSize::Dword)
}

fn vhaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 124, 52, 186], OperandSize::Dword)
}

fn vhaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 124, 232], OperandSize::Qword)
}

fn vhaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 124, 12, 145], OperandSize::Qword)
}

fn vhaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 124, 202], OperandSize::Dword)
}

fn vhaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2068749462, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 124, 20, 245, 150, 156, 78, 123], OperandSize::Dword)
}

fn vhaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 124, 223], OperandSize::Qword)
}

fn vhaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 1216556528, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 124, 161, 240, 45, 131, 72], OperandSize::Qword)
}

