use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 7, 235], OperandSize::Dword)
}

#[test]
fn vphsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1801792145, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 7, 60, 85, 145, 42, 101, 107], OperandSize::Dword)
}

#[test]
fn vphsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 7, 233], OperandSize::Qword)
}

#[test]
fn vphsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1994113198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 7, 156, 187, 174, 192, 219, 118], OperandSize::Qword)
}

#[test]
fn vphsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 7, 210], OperandSize::Dword)
}

#[test]
fn vphsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1102583832, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 7, 140, 218, 24, 24, 184, 65], OperandSize::Dword)
}

#[test]
fn vphsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 7, 202], OperandSize::Qword)
}

#[test]
fn vphsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RAX, 547208597, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 7, 152, 149, 189, 157, 32], OperandSize::Qword)
}

