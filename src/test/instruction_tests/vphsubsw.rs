use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 7, 247], OperandSize::Dword)
}

fn vphsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 983534597, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 7, 139, 5, 140, 159, 58], OperandSize::Dword)
}

fn vphsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 7, 248], OperandSize::Qword)
}

fn vphsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 44301717, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 7, 128, 149, 253, 163, 2], OperandSize::Qword)
}

fn vphsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 7, 194], OperandSize::Dword)
}

fn vphsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 41885745, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 7, 164, 150, 49, 32, 127, 2], OperandSize::Dword)
}

fn vphsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 7, 233], OperandSize::Qword)
}

fn vphsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 7, 28, 75], OperandSize::Qword)
}

