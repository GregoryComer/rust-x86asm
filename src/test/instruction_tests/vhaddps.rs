use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vhaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 124, 244], OperandSize::Dword)
}

fn vhaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 10], OperandSize::Dword)
}

fn vhaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 205], OperandSize::Qword)
}

fn vhaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1062409209, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 124, 12, 69, 249, 19, 83, 63], OperandSize::Qword)
}

fn vhaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 124, 234], OperandSize::Dword)
}

fn vhaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 231, 124, 44, 223], OperandSize::Dword)
}

fn vhaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 124, 207], OperandSize::Qword)
}

fn vhaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 124, 20, 152], OperandSize::Qword)
}

