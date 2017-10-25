use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 201], OperandSize::Dword)
}

#[test]
fn vpabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 11], OperandSize::Dword)
}

#[test]
fn vpabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 197], OperandSize::Qword)
}

#[test]
fn vpabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 60, 176], OperandSize::Qword)
}

#[test]
fn vpabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 240], OperandSize::Dword)
}

#[test]
fn vpabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1263063766, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 52, 181, 214, 210, 72, 75], OperandSize::Dword)
}

#[test]
fn vpabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 194], OperandSize::Qword)
}

#[test]
fn vpabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 22], OperandSize::Qword)
}

#[test]
fn vpabsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 29, 246], OperandSize::Dword)
}

#[test]
fn vpabsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 2023329831, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 29, 36, 125, 39, 144, 153, 120], OperandSize::Dword)
}

#[test]
fn vpabsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 137, 29, 250], OperandSize::Qword)
}

#[test]
fn vpabsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 29, 20, 113], OperandSize::Qword)
}

